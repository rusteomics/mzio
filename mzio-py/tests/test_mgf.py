from pathlib import Path
from typing import ClassVar
import unittest

from rusteomics_proteomics_io_py import mgf


class MGFModuleTestCase(unittest.TestCase):
    TEST_READ_MGF_FILE: ClassVar[Path] = Path("../test_files/mgf/Velos005137.mgf")
    TEST_WRITE_MGF_FILE: ClassVar[Path] = Path("../test_files/mgf/Velos005137.mgf.tmp")
    TEST_NON_EXISTING_MGF_FILE: ClassVar[Path] = Path("../test_files/mgf/non_existing.mgf")

    def test_read_write(self):
        reader = mgf.Reader(self.__class__.TEST_READ_MGF_FILE)

        spectra = [
            spectrum for spectrum in reader
        ]

        writer = mgf.Writer(self.__class__.TEST_WRITE_MGF_FILE)

        for spectrum in spectra:
            writer.write_spectrum(spectrum)

        writer.flush()

        del writer

        with self.__class__.TEST_READ_MGF_FILE.open("r") as in_file:
            in_file_content = in_file.read()
            with self.__class__.TEST_WRITE_MGF_FILE.open("r") as out_file:
                out_file_content = out_file.read()
                self.assertEqual(in_file_content, out_file_content)

        self.__class__.TEST_WRITE_MGF_FILE.unlink(missing_ok=True)

        with self.assertRaises(RuntimeError):
            mgf.Reader(self.__class__.TEST_NON_EXISTING_MGF_FILE)
