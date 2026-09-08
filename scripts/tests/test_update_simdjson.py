import argparse
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    'update_simdjson', Path(__file__).resolve().parents[1] / 'update_simdjson.py'
)
updater = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(updater)


class UpdateTests(unittest.TestCase):
    def test_versions_and_annotated_tags(self):
        self.assertEqual(updater.release_version('v4.6.4'), '4.6.4')
        for value in ['latest', '../4.6.4', '--help', '4.6.4;echo bad']:
            with self.assertRaises(argparse.ArgumentTypeError):
                updater.release_version(value)
        tag = 'refs/tags/v4.6.4'
        output = f'{"a" * 40}\t{tag}\n{"b" * 40}\t{tag}^{{}}\n'
        with patch.object(updater.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0, output)):
            self.assertEqual(updater.resolve_commit('4.6.4'), 'b' * 40)
        with patch.object(updater.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0, '')):
            with self.assertRaises(ValueError):
                updater.resolve_commit('4.6.4')

    def fixture(self, root):
        vendor = root / 'simdjson-sys/vendor/simdjson'
        vendor.mkdir(parents=True)
        for name, content in {
            'simdjson.h': '#define SIMDJSON_VERSION "4.6.4"\n',
            'simdjson.cpp': 'old source', 'LICENSE': 'old license',
            'README.md': 'Official singleheader sources from simdjson v4.6.4.\n',
        }.items():
            (vendor / name).write_text(content)
        (root / 'README.md').write_text('currently **v4.6.4**')
        (root / 'simdjson-sys/README.md').write_text('simdjson v4.6.4 DOM')
        (root / 'CHANGELOG.md').write_text('## [Unreleased]\n\n## Old release\nUsed v4.6.4\n')

    def download(self, commit, path):
        return {'singleheader/simdjson.h': b'#define SIMDJSON_VERSION "4.6.5"\n',
                'singleheader/simdjson.cpp': b'new source', 'LICENSE': b'new license'}[path]

    def test_update_preserves_history_and_is_repeatable(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.fixture(root)
            with patch.object(updater, 'ROOT', root), patch.object(updater, 'download', side_effect=self.download):
                updater.apply(updater.prepare(root, '4.6.5', 'a' * 40))
                history = (root / 'CHANGELOG.md').read_text()
                self.assertIn('from v4.6.4 to v4.6.5', history)
                self.assertIn('## Old release\nUsed v4.6.4', history)
                self.assertEqual((root / 'README.md').read_text(), 'currently **v4.6.5**')
                changes = updater.prepare(root, '4.6.5', 'a' * 40)
                self.assertTrue(all(path.read_bytes() == data for path, data in changes.items()))
                self.assertNotIn(root / 'CHANGELOG.md', changes)

    def test_failed_download_and_wrong_version_leave_files_untouched(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.fixture(root)
            before = {p: p.read_bytes() for p in root.rglob('*') if p.is_file()}
            def fail_on_license(commit, path):
                if path == 'LICENSE':
                    raise OSError('download interrupted')
                return self.download(commit, path)
            with patch.object(updater, 'download', side_effect=fail_on_license):
                with self.assertRaises(OSError):
                    updater.prepare(root, '4.6.5', 'a' * 40)
            with patch.object(updater, 'download', side_effect=self.download):
                with self.assertRaises(ValueError):
                    updater.prepare(root, '4.6.6', 'a' * 40)
            self.assertEqual(before, {p: p.read_bytes() for p in root.rglob('*') if p.is_file()})


if __name__ == '__main__':
    unittest.main()
