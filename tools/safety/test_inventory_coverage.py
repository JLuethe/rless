"""Module/type-only files still carry policy and must not vanish from review."""
from pathlib import Path
import copy
import json
import tempfile
import unittest

from common import sha
from inventory_format import (EXPANDED_CONFIGURATIONS, all_statics, boundary_comparison,
                              expanded_review_sites, source_inputs)
from ownership import CHECKLIST, seed, validate
from run import source_file_summaries


class SourceCoverage(unittest.TestCase):
    def test_empty_event_module_and_type_inputs_keep_hash_bound_rows(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'src').mkdir()
            (root / 'src/lib.rs').write_text('#![forbid(unsafe_code)]\npub mod types;\n')
            (root / 'src/types.rs').write_text('pub enum ResultKind { Complete, Incomplete }\n')
            (root / 'src/state.rs').write_text('static mut STATE: i32 = 0;\n')
            sources = ['src/lib.rs', 'src/types.rs', 'src/state.rs']
            result = source_file_summaries([dict(file='src/state.rs', kind='mutable_static_definition')], sources, root)
            self.assertEqual(set(result), set(sources))
            for name in sources:
                self.assertEqual(result[name]['sha256'], sha(root / name))
            self.assertEqual(result['src/lib.rs']['counts'], {})
            self.assertEqual(result['src/types.rs']['counts'], {})
            self.assertEqual(result['src/state.rs']['counts'], {'mutable_static_definition': 1})
            old = result['src/lib.rs']['sha256']
            (root / 'src/lib.rs').write_text('pub mod types;\n')
            self.assertNotEqual(source_file_summaries([], sources, root)['src/lib.rs']['sha256'], old)

    def test_missing_input_cannot_be_silently_omitted(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaises(FileNotFoundError):
                source_file_summaries([], ['missing.rs'], Path(directory))

    def test_nested_build_and_fuzz_outputs_are_not_source_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ('src/lib.rs', 'vendor/crate/src/lib.rs', 'build.rs',
                         'vendor/crate/target/debug/generated.rs',
                         'vendor/crate/artifacts/crash.rs', 'vendor/crate/corpus/seed.rs'):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('')
            self.assertEqual(source_inputs(root), ['build.rs', 'src/lib.rs', 'vendor/crate/src/lib.rs'])

    def test_symlinked_maintained_input_is_rejected_not_silently_omitted(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'src').mkdir()
            (root / 'src/alias.rs').symlink_to(root / 'outside.rs')
            with self.assertRaisesRegex(ValueError, 'symlinked'):
                source_inputs(root)


class InventoryV2Controls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        (self.root / 'src').mkdir()
        (self.root / 'src/lib.rs').write_text('static SHARED: AtomicUsize = AtomicUsize::new(0);\n')
        (self.root / 'review.txt').write_text('Independently reviewed fixture owner and expanded boundary.')
        self.records = self.static_records('src/lib.rs')
        raw = self.root / 'syntax.json'
        raw.write_text(json.dumps(self.records))
        configurations = {}
        for name, parameters in EXPANDED_CONFIGURATIONS.items():
            source = self.root / (name + '.rs')
            source.write_text('static SHARED: AtomicUsize = AtomicUsize::new(0);\n')
            syntax = self.root / (name + '.tsv')
            records = self.static_records(str(source))
            syntax.write_text('\n'.join('\t'.join(str(record[key]) for key in (
                'kind', 'file', 'line', 'column', 'context', 'symbol')) for record in records) + '\n')
            configurations[name] = dict(**copy.deepcopy(parameters),
                source=dict(path=str(source), sha256=sha(source)),
                syntax=dict(path=str(syntax), sha256=sha(syntax)),
                review_sites=expanded_review_sites(records))
        for name, config in configurations.items():
            if name.endswith('-safety-injection'):
                baseline = name.removesuffix('-safety-injection')
                config['baseline_configuration'] = baseline
                config['boundary_comparison'] = boundary_comparison(
                    configurations[baseline]['review_sites'], config['review_sites'])
        self.inventory = dict(schema_version=2,
            files=source_file_summaries(self.records, ['src/lib.rs'], self.root),
            raw_inventory=dict(path=str(raw), sha256=sha(raw)),
            mutable_static_definitions=[], static_definitions=all_statics(self.records),
            candidate_external_symbols=[], indirect_call_sites=[], expanded_configurations=configurations)
        self.regions = dict(regions=[dict(id='OWN-FIXTURE', current_paths=['src/lib.rs'], delivery_packet='M0.2')])
        self.findings = dict(findings=[])

    @staticmethod
    def static_records(path):
        location = dict(file=path, line=1, column=1, context='')
        return [dict(**location, kind='static_definition', symbol='SHARED'),
                dict(**location, kind='static_declared_type', symbol='AtomicUsize')]

    def tearDown(self):
        self.temp.cleanup()

    def check(self, review, complete=False):
        return validate(review, self.inventory, self.regions, self.findings, self.root, complete=complete)

    def completed_review(self):
        review = seed(self.inventory, self.regions, self.root)
        static = next(item for item in review['entries'] if item['kind'] == 'shared-static-definition')
        for item in review['entries']:
            item.update(status='reviewed-clear', reviewer='Independent Fixture Reviewer', packet='M0.2',
                evidence_paths=['review.txt'], evidence_sha256={'review.txt': sha(self.root / 'review.txt')},
                checklist={field: 'Fixture source-bound analysis.' for field in CHECKLIST})
            if item['kind'] == 'expanded-source':
                name = item['configuration']
                path = self.root / (name + '-review.json')
                path.write_text(json.dumps(dict(configuration=name,
                    source_sha256=self.inventory['expanded_configurations'][name]['source']['sha256'],
                    site_reviews=[dict(site_ids=item['site_ids'], disposition='source-covered',
                        source_review_ids=[static['id']], analysis='The exact SHARED static is covered by its source owner review.')])))
                item['expanded_review_map'] = dict(path=path.name, sha256=sha(path))
                item['evidence_paths'].append(path.name)
                item['evidence_sha256'][path.name] = sha(path)
        return review

    def test_shared_static_has_review_row_and_omission_is_rejected(self):
        review = seed(self.inventory, self.regions, self.root)
        statics = [item for item in review['entries'] if item['kind'] == 'shared-static-definition']
        self.assertEqual([(item['symbol'], item['declared_type']) for item in statics], [('SHARED', 'AtomicUsize')])
        self.assertEqual(self.check(review)['status'], 'not-verified')
        review['entries'].remove(statics[0])
        with self.assertRaisesRegex(ValueError, 'missing, extra'):
            self.check(review)
        self.inventory['static_definitions'] = []
        with self.assertRaisesRegex(ValueError, 'all-static'):
            seed(self.inventory, self.regions, self.root)

    def test_omitted_feature_configuration_or_site_cannot_be_reseeded_as_complete(self):
        original = copy.deepcopy(self.inventory)
        self.inventory['expanded_configurations'].pop('lib-safety-injection')
        with self.assertRaisesRegex(ValueError, 'expanded build configuration'):
            seed(self.inventory, self.regions, self.root)
        self.inventory = original
        self.inventory['expanded_configurations']['lib-safety-injection']['review_sites'] = []
        with self.assertRaisesRegex(ValueError, 'expanded review site'):
            seed(self.inventory, self.regions, self.root)

    def test_changed_expansion_and_missing_new_source_invalidate_inventory(self):
        source = Path(self.inventory['expanded_configurations']['lib']['source']['path'])
        old = source.read_text()
        source.write_text('changed expanded source')
        with self.assertRaisesRegex(ValueError, 'stale inventory artifact'):
            seed(self.inventory, self.regions, self.root)
        source.write_text(old)
        (self.root / 'src/new.rs').write_text('static NEW: usize = 0;')
        with self.assertRaisesRegex(ValueError, 'source input'):
            seed(self.inventory, self.regions, self.root)

    def test_positive_complete_mapping_and_missing_mapping_negative(self):
        review = self.completed_review()
        self.assertEqual(self.check(review, complete=True)['status'], 'reviewed')
        item = next(item for item in review['entries'] if item['kind'] == 'expanded-source')
        item.pop('expanded_review_map')
        with self.assertRaisesRegex(ValueError, 'per-site review map'):
            self.check(review, complete=True)

    def test_site_mapping_requires_complete_source_review_and_exact_coverage(self):
        review = self.completed_review()
        source = next(item for item in review['entries'] if item['kind'] == 'shared-static-definition')
        source['status'] = 'pending'
        with self.assertRaisesRegex(ValueError, 'unreviewed or imprecise'):
            self.check(review)
        source['status'] = 'reviewed-clear'
        item = next(item for item in review['entries'] if item['kind'] == 'expanded-source')
        path = self.root / item['expanded_review_map']['path']
        content = json.loads(path.read_text())
        content['site_reviews'] = []
        path.write_text(json.dumps(content))
        item['expanded_review_map']['sha256'] = sha(path)
        item['evidence_sha256'][path.name] = sha(path)
        with self.assertRaisesRegex(ValueError, 'unreviewed expanded'):
            self.check(review)

    def test_feature_comparison_tracks_added_removed_and_matching_boundaries(self):
        base = expanded_review_sites(self.records)
        added_records = self.static_records('expanded.rs')
        added_records[0]['symbol'] = 'FEATURE_ONLY'
        added = expanded_review_sites(added_records)
        comparison = boundary_comparison(base, base + added)
        self.assertEqual(comparison['added_site_ids'], [added[0]['id']])
        self.assertEqual(comparison['removed_site_ids'], [])
        self.assertEqual(comparison['matched'][0]['base_site_ids'], [base[0]['id']])
        self.assertEqual(boundary_comparison(base + added, base)['removed_site_ids'], [added[0]['id']])


if __name__ == '__main__':
    unittest.main(verbosity=2)
