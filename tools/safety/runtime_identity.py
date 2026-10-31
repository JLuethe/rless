#!/usr/bin/env python3
"""Bind a pager runtime observation to its actual mapped GNU libc ELF bytes."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess

from common import ROOT, sha, write_json
from linked_object import Elf


def require(value, message):
    if not value: raise ValueError(message)


def interpreter(binary):
    data=Path(binary).read_bytes()
    require(data[:7]==b'\x7fELF\x02\x01\x01','unsupported runtime ELF')
    header=struct.unpack_from('<16sHHIQQQIHHHHHH',data)
    offset,size,count=header[5],header[9],header[10]
    require(size==56 and offset+size*count<=len(data),'invalid ELF program headers')
    values=[]
    for index in range(count):
        kind,flags,file_offset,address,physical,file_size,memory_size,alignment=struct.unpack_from('<IIQQQQQQ',data,offset+index*size)
        if kind==3:
            require(0<file_size<4096 and file_offset+file_size<=len(data),'invalid interpreter extent')
            text=data[file_offset:file_offset+file_size]
            require(text.endswith(b'\0') and text.count(b'\0')==1,'invalid interpreter string')
            values.append(text[:-1].decode())
    require(values==['/lib64/ld-linux-x86-64.so.2'],'unexpected runtime interpreter')
    return values[0]


def libc_version(path):
    """Read the actual constant returned by x86_64 gnu_get_libc_version.

    This never loads or executes a submitted DSO. Unknown getter code fails.
    """
    path=Path(path);elf=Elf(path.read_bytes(),{3})
    text=subprocess.check_output(['readelf','--dyn-syms','--wide',str(path)],text=True,env={'LC_ALL':'C'})
    matches=[]
    for line in text.splitlines():
        fields=line.split()
        if len(fields)>=8 and fields[3]=='FUNC' and fields[6]!='UND' and fields[7].split('@')[0]=='gnu_get_libc_version':
            matches.append((int(fields[1],16),int(fields[2])))
    require(len(set(matches))==1,'missing or ambiguous libc version getter')
    address,size=matches[0];code=elf.address_bytes(address,size)
    start=4 if code.startswith(b'\xf3\x0f\x1e\xfa') else 0
    require(code[start:start+3]==b'\x48\x8d\x05' and code[start+7:start+8]==b'\xc3','unsupported libc version getter instructions')
    target=address+start+7+struct.unpack_from('<i',code,start+3)[0]
    # Read only mapped source bytes, stopping at the first string terminator.
    raw=bytearray()
    for index in range(32):
        byte=elf.address_bytes(target+index,1)
        if byte==b'\0':break
        raw.extend(byte)
    version=bytes(raw).decode('ascii')
    require(re.fullmatch(r'2\.\d+',version),'invalid libc version constant')
    return dict(version=version,getter_address=address,getter_code_sha256=hashlib.sha256(code).hexdigest())


def libc_mappings(text):
    found=[]
    for line in text.splitlines():
        fields=line.split(maxsplit=5)
        if len(fields)==6 and fields[5].endswith('/libc.so.6'):
            found.append(dict(range=fields[0],permissions=fields[1],offset=fields[2],device=fields[3],inode=int(fields[4]),path=fields[5]))
    require(found and len({x['path'] for x in found})==1 and any('x' in x['permissions'] for x in found),'mapped executable libc missing or ambiguous')
    return found


def reference_context(root=ROOT):
    """Context only for an actual reference runtime; outer verifier pins image ID.

    The current process's libc bytes must be 2.41, and the immutable image
    runtime manifest must match the maintained lock. No report-supplied bit or
    getconf text can create this context.
    """
    root=Path(root)
    lock_path=root/'docs/verification/glibc241-environment.json'
    lock=json.loads(lock_path.read_text())
    runtime=Path('/opt/rless-abi/runtime.txt')
    require(runtime.is_file() and runtime.read_text().strip()==lock['runtime'],'reference image runtime manifest differs from maintained lock')
    require(sha(root/'tools/safety/glibc241/Containerfile')==lock['containerfile_sha256'],'reference image procedure changed')
    provider=Path(libc_mappings(Path('/proc/self/maps').read_text())[0]['path'])
    require(libc_version(provider)['version']=='2.41','verifier itself is not executing the reference libc')
    return dict(validated_reference_image=True,libc_sha256=sha(provider),image_id=lock['image_id'],
                image_rootfs=lock['rootfs'],environment_lock_sha256=sha(lock_path),runtime_manifest_sha256=sha(runtime))


def validate(result, directory, binary, *, abi_profile, reference_context=None):
    directory,binary=Path(directory),Path(binary)
    require(result.get('binary_sha256')==sha(binary),'runtime executable changed')
    maps=directory/'maps.raw';libc=directory/'runtime-libc.so.6'
    require(result.get('maps_sha256')==sha(maps) and result.get('libc_sha256')==sha(libc),'runtime mapping/libc artifact changed')
    require(result.get('mappings')==libc_mappings(maps.read_text()),'runtime mappings differ from retained raw maps')
    proof=libc_version(libc)
    require(result.get('libc_version_proof')==proof,'runtime libc version proof differs from actual ELF')
    expected={'BUILDER-GLIBC-2.44':'2.44','REFERENCE-GLIBC-2.41':'2.41'}
    require(abi_profile in expected and proof['version']==expected[abi_profile],'ABI label differs from mapped libc artifact')
    require(result.get('interpreter')==interpreter(binary),'runtime interpreter differs from actual ELF')
    versions=subprocess.check_output(['readelf','--version-info','--wide',str(binary)],text=True,env={'LC_ALL':'C'})
    needed=sorted(set(re.findall(r'Name: (GLIBC_\d+\.\d+(?:\.\d+)?)',versions)))
    require(needed and result.get('glibc_requirements')==needed,'runtime GLIBC requirements changed')
    maximum=max(tuple(map(int,x.removeprefix('GLIBC_').split('.'))) for x in needed)
    require(maximum<=tuple(map(int,proof['version'].split('.'))),'executable requires newer glibc than observed provider')
    observed=result.get('observation',{})
    require(observed.get('status')==0 and observed.get('termios_restored') is True,'runtime probe did not exit with restored terminal')
    require(result.get('observer_sha256')==sha(ROOT/'tools/safety/runtime_identity.py'),'runtime observer changed')
    if abi_profile=='REFERENCE-GLIBC-2.41':
        lock_path=ROOT/'docs/verification/glibc241-environment.json';lock=json.loads(lock_path.read_text())
        require(isinstance(reference_context,dict) and reference_context.get('validated_reference_image') is True
                and reference_context.get('libc_sha256')==sha(libc)
                and reference_context.get('image_id')==lock['image_id']
                and reference_context.get('image_rootfs')==lock['rootfs']
                and reference_context.get('environment_lock_sha256')==sha(lock_path),
                'reference ABI requires independently validated image/libc context')
    return proof


def observe(binary, output):
    from test_io_boundaries import pager
    from run_v2 import run_case
    binary,output=Path(binary).resolve(strict=True),Path(output).resolve()
    output.mkdir(parents=True,exist_ok=False)
    captured={}
    def checkpoint(pid,label):
        require(label=='runtime' and not captured,'duplicate runtime checkpoint')
        process=Path('/proc')/str(pid)
        require(sha(process/'exe')==sha(binary),'runtime probe executes a different ELF')
        text=(process/'maps').read_text();rows=libc_mappings(text)
        provider=process/'root'/rows[0]['path'].lstrip('/')
        info=provider.stat()
        require(info.st_ino==rows[0]['inode'],'runtime libc mapping/file identity differs')
        (output/'maps.raw').write_text(text)
        (output/'runtime-libc.so.6').write_bytes(provider.read_bytes())
        captured.update(mappings=rows,maps_sha256=sha(output/'maps.raw'),libc_sha256=sha(output/'runtime-libc.so.6'))
    case=pager(events=[{'await':['READY'],'heap_checkpoint':'runtime'},{'send':'q'}])
    observed=run_case(binary,case,checkpoint_observer=checkpoint)
    require(captured,'runtime checkpoint missing')
    versions=subprocess.check_output(['readelf','--version-info','--wide',str(binary)],text=True,env={'LC_ALL':'C'})
    result=dict(captured,binary_sha256=sha(binary),libc_version_proof=libc_version(output/'runtime-libc.so.6'),
        interpreter=interpreter(binary),glibc_requirements=sorted(set(re.findall(r'Name: (GLIBC_\d+\.\d+(?:\.\d+)?)',versions))),
        observer_sha256=sha(__file__),observation=observed)
    write_json(output/'result.json',result)
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',type=Path,required=True);parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();result=observe(args.binary,args.output)
    print(json.dumps(dict(libc=result['libc_version_proof']['version'],binary_sha256=result['binary_sha256'])))


if __name__=='__main__':main()
