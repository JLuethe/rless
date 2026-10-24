# Initial leak reproducers

These are reduced additional diagnostic cases, separate from the frozen corpus:

| Finding | Case | Complete representative report |
| --- | --- | --- |
| [#9 GNU regex match registers](https://github.com/JLuethe/rless/issues/9) | [m0-leak-regex.json](m0-leak-regex.json) | [LeakSanitizer stack](m0-leak-regex.log) |
| [#10 startup command storage](https://github.com/JLuethe/rless/issues/10) | [m0-leak-command.json](m0-leak-command.json) | [LeakSanitizer stack](m0-leak-command.log) |

Both use a two-byte file `x\n`, a real PTY/readiness checkpoint and `q`. On the
pinned initial ASan/leak build both exit 87 with the intended leak diagnostic.
They are retained failing reproductions, not changed expected observations or
passing safety acceptance tests. Reports contain the original symbolized stack.

First build the pinned instrumented candidate with the initial lane documented in
[the tool instructions](../../../tools/safety/README.md). Its aggregate remains
nonzero while findings/coverage gaps are open. Then, from the repository root,
with the Python environment active, replay the reduced cases:

```sh
python - <<'PY'
import json, os, sys, tempfile
from pathlib import Path
sys.path.insert(0, 'tools/compat')
sys.path.insert(0, 'tools/safety')
from run_v2 import run_case
from common import PINS
# Replace this with your initial lane's instrumented binary path.
binary = Path('compat/.runs/m0-initial-2/target/x86_64-unknown-linux-gnu/debug/rless').resolve()
report = Path(tempfile.mkdtemp(prefix='leak-repro-', dir='compat/.runs')).resolve()
config = report / 'environment.json'
config.write_text(json.dumps({
    'ASAN_OPTIONS': PINS['asan_options'] + ':log_path=' + str(report / 'asan'),
    'LSAN_OPTIONS': PINS['lsan_options'],
    'ASAN_SYMBOLIZER_PATH': '/usr/bin/llvm-symbolizer',
}))
os.environ['RLESS_SAFETY_ENV'] = str(config)
for path in sorted(Path('docs/safety/reproducers').glob('*.json')):
    for case in json.loads(path.read_text()):
        actual = run_case(binary, case)
        print(case['id'], 'program status:', actual['status'])
print('Original leak reports:', report)
PY
```

The tool runner verifies the symbolizer pin; use that verified executable when
reproducing on another host. Retain the original process statuses and diagnostics.
A reference output match or a suppressed detector is not a fix.
