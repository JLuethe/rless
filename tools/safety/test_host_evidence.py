"""Mutation controls for host baseline adaptation; fixtures are not product proof."""
import copy
import hashlib
import io
import json
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from host_evidence import (PROCEDURE, checked, normalize, validate_commands,
                           validate_external_report, validate_source_archive,
                           dynamic_symbols, validate_terminfo_provider)
from common import sha


class HostEvidenceControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.out = self.root/'retained'
        self.out.mkdir()
        self.source = self.out/'source'
        self.rebuilt = self.out/'source-rebuild/candidate'
        self.source.mkdir()
        self.rebuilt.mkdir(parents=True)

    def tearDown(self):
        self.temp.cleanup()

    def reference(self, path, content='Fixture evidence\n'):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
        return dict(path=str(path), sha256=sha(path))

    def command_report(self):
        expected = {
            'package': [sys.executable, str(self.source/'tools/package.py')],
            'source-build': ['cargo','build','--release','--locked','--offline','--bins'],
            'source-tests': ['cargo','test','--release','--locked','--offline'],
            'source-acceptance': [sys.executable,str(self.rebuilt/'tools/compat/check_candidate.py')],
            'abi-build': ['cc','-Wall','-Wextra','-Werror',str(self.source/'tools/safety/abi_probe.c'),'-o',str(self.out/'abi-probe')],
            'abi-probe': [str(self.out/'abi-probe')],
            'abi-headers': ['cc','-M',str(self.source/'tools/safety/abi_probe.c')],
            'version-rustc': ['rustc','-vV'], 'version-cargo': ['cargo','-V'],
            'version-cc': ['cc','--version'], 'version-linker': ['ld','--version'],
            'version-glibc': ['getconf','GNU_LIBC_VERSION'],
            'path-rustc': ['rustup','which','rustc'], 'path-cargo': ['rustup','which','cargo'],
            'ldd-reference': ['ldd','fixture-reference'], 'ldd-debug': ['ldd','fixture-debug'],
            'ldd-release': ['ldd','fixture-release'], 'terminfo': ['infocmp','-x','xterm-256color']}
        return dict(rust_tool_resolution={name:dict(resolver='rustup') for name in ('rustc','cargo')},
            commands=[dict(name=name, command=command,
            cwd=str(self.rebuilt if name.startswith('source-') else self.source),
            original_exit_status=0, expected_exit_status=0,
            log=self.reference(self.out/(name+'.log'))) for name,command in expected.items()])

    def test_historical_or_incomplete_pass_boolean_gets_no_host_credit(self):
        path = self.out/'summary.json'
        for report in [dict(schema_version=1,status='passed'), dict(schema_version=2,report_kind='host-native-baseline',
                procedure=PROCEDURE,source_kind='committed-source',status='passed',finished_complete=False),
                dict(schema_version=2,report_kind='host-native-baseline',procedure=PROCEDURE,source_kind='committed-source',
                status='passed',finished_complete=True,original_exit_status=False,runtime_safety_verified=False,
                started_at_utc='fixture',finished_at_utc='fixture')]:
            with self.subTest(report=report):
                path.write_text(json.dumps(report))
                with self.assertRaises(ValueError):
                    validate_external_report(path,self.root)

    def test_retained_references_reject_changes_links_and_escapes(self):
        value = self.reference(self.out/'artifact')
        self.assertEqual(checked(value,within=self.out),self.out/'artifact')
        outside = self.reference(self.root/'outside')
        with self.assertRaisesRegex(ValueError,'escaped'):
            checked(outside,within=self.out)
        link = self.out/'link'
        link.symlink_to(self.out/'artifact')
        with self.assertRaisesRegex(ValueError,'linked'):
            checked(dict(value,path=str(link)))
        (self.out/'artifact').write_text('changed')
        with self.assertRaisesRegex(ValueError,'changed'):
            checked(value)

    def test_command_scope_status_and_artifact_omissions_fail(self):
        original = self.command_report()
        self.assertEqual(len(validate_commands(original,self.out,self.source,self.rebuilt)),18)
        for mutation in ('missing-build','duplicate','boolean-status','failed-status','wrong-cwd','skip-tests','no-offline'):
            with self.subTest(mutation=mutation):
                report = copy.deepcopy(original)
                step = next(item for item in report['commands'] if item['name']=='source-build')
                if mutation=='missing-build':
                    report['commands'].remove(step)
                elif mutation=='duplicate':
                    report['commands'].append(copy.deepcopy(step))
                elif mutation=='boolean-status':
                    step['original_exit_status']=False
                elif mutation=='failed-status':
                    step['original_exit_status']=1
                elif mutation=='wrong-cwd':
                    step['cwd']=str(self.root)
                elif mutation=='skip-tests':
                    next(item for item in report['commands'] if item['name']=='source-tests')['command']=['true']
                else:
                    step['command'].remove('--offline')
                with self.assertRaises(ValueError):
                    validate_commands(report,self.out,self.source,self.rebuilt)
        (self.out/'abi-probe.log').unlink()
        with self.assertRaises(ValueError):
            validate_commands(original,self.out,self.source,self.rebuilt)

    def test_sanitizer_diagnostic_cannot_hide_behind_zero_exit_and_fresh_hash(self):
        report=self.command_report()
        step=next(item for item in report['commands'] if item['name']=='source-tests')
        step['log']=self.reference(self.out/'source-tests.log','ERROR: AddressSanitizer: heap-use-after-free\n')
        with self.assertRaisesRegex(ValueError,'diagnostic'):
            validate_commands(report,self.out,self.source,self.rebuilt)

    def test_archive_bytes_modes_and_exact_selection_are_rederived(self):
        content=b'fixture source\n'
        manifest=dict(files={'src/main.rs':dict(sha256=hashlib.sha256(content).hexdigest(),mode=0o644)})
        archive=self.out/'source.tar'
        def write(mutation=None):
            with tarfile.open(archive,'w') as target:
                member=tarfile.TarInfo('src/main.rs')
                member.mode=0o755 if mutation=='mode' else 0o644
                data=b'changed source\n' if mutation=='bytes' else content
                member.size=len(data)
                if mutation=='link':
                    member.type=tarfile.SYMTYPE
                    member.linkname='/etc/passwd'
                target.addfile(member,io.BytesIO(data) if member.isfile() else None)
                if mutation in ('extra','duplicate'):
                    extra=tarfile.TarInfo('private.txt' if mutation=='extra' else 'src/main.rs')
                    extra.size=0
                    target.addfile(extra,io.BytesIO())
        write()
        validate_source_archive(archive,manifest)
        for mutation in ('bytes','mode','link','extra','duplicate'):
            with self.subTest(mutation=mutation):
                write(mutation)
                with self.assertRaises(ValueError):
                    validate_source_archive(archive,manifest)

    def test_normalized_credit_is_scoped_and_never_claims_runtime_safety(self):
        self.reference(self.root/'compat/corpus.json','{"files":{}}')
        log=self.out/'source-acceptance.log'
        self.reference(log)
        value=normalize('acceptance-source-rebuild',log,self.root)
        self.assertEqual(value['_validated_m0_controls'],['acceptance-source-rebuild'])
        self.assertEqual(value['oracle_metadata']['abi_profile'],'BUILDER-GLIBC-2.44')
        self.assertEqual(value['oracle_metadata']['executed_cases'],809)
        self.assertEqual(value['coverage_scope'],'partial')
        self.assertNotIn('runtime_safety_verified',value)

    def test_loaded_terminfo_provider_requires_exact_import_definitions(self):
        binary = self.root/'pager'
        provider = self.reference(self.root/'libncursesw.so.6.6')
        objects = {str(self.root/'libncursesw.so.6'): provider}
        imports = {'setupterm@TERM_1', 'tputs@TERM_1', 'tigetstr@TERM_1'}
        exports = set(imports)
        def symbols(path, reader):
            return (imports, set()) if path == binary else (set(), exports)
        with patch('host_evidence.dynamic_symbols', side_effect=symbols):
            proof = validate_terminfo_provider(binary, objects, '/reader')
            self.assertEqual(proof['symbols'], sorted(imports))
            for mutation in ('missing', 'wrong-version', 'wrong-provider-name', 'unloaded', 'no-imports', 'changed-object'):
                with self.subTest(mutation=mutation):
                    chosen = objects
                    exports = set(imports)
                    if mutation == 'missing':
                        exports.remove('tputs@TERM_1')
                    elif mutation == 'wrong-version':
                        exports = {v.replace('TERM_1', 'TERM_2') for v in exports}
                    elif mutation == 'wrong-provider-name':
                        chosen = {str(self.root/'libpretend.so.6'): provider}
                    elif mutation == 'unloaded':
                        chosen = {}
                    elif mutation == 'no-imports':
                        imports = {'malloc@GLIBC_2.2.5'}
                    elif mutation == 'changed-object':
                        imports = {'setupterm@TERM_1', 'tputs@TERM_1', 'tigetstr@TERM_1'}
                        Path(provider['path']).write_text('different')
                    with self.assertRaises(ValueError):
                        validate_terminfo_provider(binary, chosen, '/reader')

    def test_symbol_reader_distinguishes_definitions_and_default_versions(self):
        from types import SimpleNamespace
        output = '''Symbol table '.dynsym' contains 6 entries:
  1: 00000000 0 FUNC GLOBAL DEFAULT UND tputs@TERM_1 (2)
  2: 00000010 5 FUNC GLOBAL DEFAULT 12 setupterm@@TERM_1
  3: 00000020 5 FUNC GLOBAL DEFAULT 12 tputs@TERM_0
  4: 00000030 5 FUNC LOCAL DEFAULT 12 hidden
  5: 00000040 5 FUNC GLOBAL HIDDEN 12 concealed
'''
        with patch('host_evidence.subprocess.run', return_value=SimpleNamespace(
                returncode=0, stderr='', stdout=output)) as run:
            imported, exported = dynamic_symbols('/pager', '/reader')
        self.assertEqual(imported, {'tputs@TERM_1'})
        self.assertEqual(exported, {'setupterm@TERM_1', 'setupterm', 'tputs@TERM_0'})
        self.assertEqual(run.call_args.args[0], ['/reader','--dyn-syms','--wide','/pager'])


if __name__=='__main__':
    unittest.main(verbosity=2)
