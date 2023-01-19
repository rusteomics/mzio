from pathlib import Path
from typing import ClassVar
import unittest

from rusteomics_proteomics_io_py import fasta

class FastaModuleTestCase(unittest.TestCase):
    TEST_READ_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/mouse.fasta")
    TEST_WRITE_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/mouse.fasta.tmp")
    TEST_NON_EXISTING_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/non_existing.fasta")

    def test_read_write(self):
        reader = fasta.Reader(self.__class__.TEST_READ_FASTA_FILE)

        entries = [
            entry for entry in reader 
        ]

        writer = fasta.Writer(self.__class__.TEST_WRITE_FASTA_FILE)

        for entry in entries:
            writer.write_entry(entry, True)

        writer.flush()

        del writer

        with self.__class__.TEST_READ_FASTA_FILE.open("r") as in_file:
            in_file_content = in_file.read()
            with self.__class__.TEST_WRITE_FASTA_FILE.open("r") as out_file:
                out_file_content = out_file.read()
                self.assertEqual(in_file_content, out_file_content)

        self.__class__.TEST_WRITE_FASTA_FILE.unlink(missing_ok=True)

        with self.assertRaises(RuntimeError):
            fasta.Reader(self.__class__.TEST_NON_EXISTING_FASTA_FILE)
