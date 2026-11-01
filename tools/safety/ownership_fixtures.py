"""Structured ownership packets for synthetic unit fixtures only; no approvals."""
import hashlib
from pathlib import Path
from common import sha, write_json
from ownership import CHECKLIST, REVIEW_PACKET_KIND, review_target


def packet(item, review, inventory, root):
    name = item['paths'][0] if item['paths'] else next(iter(inventory['files']))
    source = root/name
    if item['kind'] == 'expanded-source':
        source = root/Path(inventory['expanded_configurations'][item['configuration']]['source']['path'])
        target = 'expanded:'+item['configuration']
    else:
        target = 'source:'+name
    content=source.read_text();lines=content.splitlines(keepends=True)
    citation=dict(target=target,sha256=sha(source),start_line=1,end_line=len(lines),
                  excerpt_sha256=hashlib.sha256(content.encode()).hexdigest())
    target_name=item.get('symbol') or item['id'].removeprefix('FILE:')
    checklist={}
    for index,key in enumerate(CHECKLIST):
        end=min(len(lines),index+1)
        excerpt=''.join(lines[:end]).encode()
        citation=dict(target=target,sha256=sha(source),start_line=1,end_line=end,
                      excerpt_sha256=hashlib.sha256(excerpt).hexdigest())
        checklist[key]=dict(analysis=f'The synthetic {key} contract for {target_name} is fixture-only; it makes no production approval.',
            invariant='This invariant belongs only to the explicitly synthetic test fixture.',source_refs=[citation],
            limitations=['This is synthetic fixture evidence, not a production review.'] if index==0 else [])

    item.update(checklist=checklist)
    path='docs/safety/ownership-evidence/'+hashlib.sha256(item['id'].encode()).hexdigest()+'.json'
    write_json(root/path,dict(schema_version=1,review_kind=REVIEW_PACKET_KIND,target=review_target(item),
        inventory_sha256=review['inventory_sha256'],reviewer=item['reviewer'],checklist=checklist))
    item['review_packet']=dict(path=path,sha256=sha(root/path))
    item['evidence_paths'].append(path);item['evidence_sha256'][path]=sha(root/path)
