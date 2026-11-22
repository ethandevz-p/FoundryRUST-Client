"""Crate renames must not rename player data paths or the separate registry repo."""
from contextlib import redirect_stdout
import importlib.util
import io
from pathlib import Path
import shutil
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("rename_crate", Path(__file__).resolve().parents[1] / "tools/rename_crate.py")
rename = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rename)


class RenameCrateTests(unittest.TestCase):
    def test_manifest_and_crate_paths_change_without_touching_player_paths(self):
        source = '''[package]
name = "foundry-mods"
[dependencies]
foundry-mods = { path = "../foundry-mods" }
[dev-dependencies.foundry-mods]
path = "../crates/foundry-mods"
# Put files in foundry-mods/ and enable them in foundry-mods.txt.
'''
        expected = '''[package]
name = "foundry-mod-package"
[dependencies]
foundry-mod-package = { path = "../foundry-mod-package" }
[dev-dependencies.foundry-mod-package]
path = "../crates/foundry-mod-package"
# Put files in foundry-mods/ and enable them in foundry-mods.txt.
'''
        self.assertEqual(rename.rewrite("crates/foundry-mods/Cargo.toml", source), expected)
        self.assertEqual(rename.destination("crates/foundry-mods/tests/file.hmod"),
                         "crates/foundry-mod-package/tests/file.hmod")
        self.assertEqual(rename.destination("crates/foundry-mods-other/src/lib.rs"),
                         "crates/foundry-mods-other/src/lib.rs")
        self.assertEqual(rename.rewrite("crates/foundry-mod-registry/tests/submit.rs", '../foundry-mods/tests/index'),
                         '../foundry-mod-package/tests/index')

    def test_rust_library_identifiers_and_link_arguments_change(self):
        source = '''use foundry_mods::{Content, Side};
mod foundry_mods { fn fixture() {} }
let link = "foundry_mods={}";
let module = foundry_mods::Package;
fn foundry_mods_test_name() {}
let folder = "foundry-mods";
let registry = "https://github.com/foundry-rust/foundry-mods";
'''
        expected = '''use foundry_mod_package::{Content, Side};
mod foundry_mod_package { fn fixture() {} }
let link = "foundry_mod_package={}";
let module = foundry_mod_package::Package;
fn foundry_mods_test_name() {}
let folder = "foundry-mods";
let registry = "https://github.com/foundry-rust/foundry-mods";
'''
        self.assertEqual(rename.rewrite("launcher/src/native_mods/prebuilt.rs", source), expected)

    def test_generated_manifest_packages_do_not_rename_player_or_negative_strings(self):
        source = '''for name in [
    "foundry-mods",
] { let dependency = path_dep(&format!("../{name}")); }
let folder = "foundry-mods";
assert!(!dependencies.contains_key("foundry-mods"));
'''
        expected = '''for name in [
    "foundry-mod-package",
] { let dependency = path_dep(&format!("../{name}")); }
let folder = "foundry-mods";
assert!(!dependencies.contains_key("foundry-mods"));
'''
        self.assertEqual(rename.rewrite("launcher/src/native_mods/api.rs", source), expected)
        self.assertEqual(rename.rewrite("launcher/src/export.rs", source), source)

    @unittest.skipUnless(shutil.which("rustfmt"), "requires rustfmt")
    def test_verify_rejects_an_unchanged_file_that_still_needs_the_mapping(self):
        with TemporaryDirectory() as directory, patch.object(rename, "ROOT", Path(directory)):
            crate = rename.ROOT / rename.OLD_PATH
            crate.mkdir(parents=True)
            (crate / "Cargo.toml").write_text('[package]\nname = "foundry-mods"\n')
            consumer = rename.ROOT / "consumer.rs"
            consumer.write_text("use foundry_mods::Manifest;\n")
            rename.run("git", "init", "-q")
            rename.run("git", "add", "--", rename.OLD_PATH, "consumer.rs")
            rename.run("git", "-c", "user.name=Test", "-c", "user.email=test@example.invalid",
                       "commit", "-qm", "Baseline")
            rename.run("git", "mv", rename.OLD_PATH, rename.NEW_PATH)
            (rename.ROOT / rename.NEW_PATH / "Cargo.toml").write_text(
                '[package]\nname = "foundry-mod-package"\n', newline="\n")
            with self.assertRaisesRegex(AssertionError, "unexpected content change: consumer.rs"):
                rename.verify("HEAD")
            consumer.write_text("use foundry_mod_package::Manifest;\n", newline="\n")
            with redirect_stdout(io.StringIO()):
                rename.verify("HEAD")

    def test_book_filters_and_translations_obey_the_boundary(self):
        source = '{{#include ../../crates/foundry-mods/src/lib.rs}}\n(crates/foundry-mods)\ncrates/foundry-mods/**\n'
        expected = '{{#include ../../crates/foundry-mod-package/src/lib.rs}}\n(crates/foundry-mod-package)\ncrates/foundry-mod-package/**\n'
        self.assertEqual(rename.rewrite("docs/modding/src/reference.md", source), expected)
        translation = '{"native.mods": "In foundry-mods; internal type foundry_mods::Package"}'
        self.assertEqual(rename.rewrite("crates/foundry-client/assets/foundry/lang/en_us.json", translation), translation)
        self.assertEqual(rename.rewrite("roadmap/roadmap.toml", 'dev = "foundry-mods/native/"'),
                         'dev = "foundry-mods/native/"')


if __name__ == "__main__":
    unittest.main()
