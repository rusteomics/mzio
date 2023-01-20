# std imports
from itertools import pairwise
from pathlib import Path
import re
from typing import ClassVar, List, Tuple, Type
import unittest

# internal imports
from mzio_py import fasta


class FastaModuleTestCase(unittest.TestCase):
    TEST_READ_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/mouse.fasta")
    TEST_WRITE_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/mouse.fasta.tmp")
    TEST_NON_EXISTING_FASTA_FILE: ClassVar[Path] = Path("../test_files/fasta/non_existing.fasta")

    READER_WRITER_PAIRS: ClassVar[List[Tuple[Type, Type]]] = [
        (fasta.PlainReader, fasta.PlainWriter),
        (fasta.UniProtReader, fasta.UniProtWriter)
    ]

    def test_read_write_plain(self):
        reader = fasta.PlainReader(self.__class__.TEST_READ_FASTA_FILE, 1024)

        entries = [
            entry for entry in reader 
        ]

        writer = fasta.PlainWriter(self.__class__.TEST_WRITE_FASTA_FILE)

        for entry in entries:
            writer.write_entry(entry)

        writer.flush()

        del writer

        with self.__class__.TEST_READ_FASTA_FILE.open("r") as in_file:
            in_file_content = in_file.read()
            with self.__class__.TEST_WRITE_FASTA_FILE.open("r") as out_file:
                out_file_content = out_file.read()
                self.assertEqual(in_file_content, out_file_content)

        self.__class__.TEST_WRITE_FASTA_FILE.unlink(missing_ok=True)

        with self.assertRaises(RuntimeError):
            fasta.PlainReader(self.__class__.TEST_NON_EXISTING_FASTA_FILE, 1024)

    def __split_keyword_attributes(self, keyword_attributes: str) -> List[str]:
        """Simple function to split the keyword attributes of a UniProt header line.

        Parameters
        ----------
        keyword_attributes : str
            Keyword attributes of FASTA header, e.g. `OS=Zika virus (isolate ZIKV/Human/French Polynesia/10087PF/2013) OX=2043570 PE=1 SV=1`

        Returns
        -------
        List[str]
            List of keyword attributes, e.g. `["OS=Zika virus (isolate ZIKV/Human/French Polynesia/10087PF/2013)", "OX=2043570", "PE=1", "SV=1"]`
        """
        # keep it simple for testing and don't rely on complex regular expressions.
        # just find the start of each keyword attribute (`SOMEKEY=`) and slice them.
        start_positions: List[int] = [match.start() for match in re.finditer(r"[A-Z]+=", keyword_attributes)]
        # add the end of the line
        start_positions.append(len(keyword_attributes))
        return [ keyword_attributes[start:end].strip() for (start, end) in pairwise(start_positions) ]

    def test_read_write_uniprot(self):
        """
        Tests UniProtReader.
        The rust implementation of the UniProt entry will sort the keyword arguments in test mode.
        Unfortunately, it is not passed down to the imported `mzio` crate. Therefore, the test cannot compare the lines directly
        and has to split the keyword attributes and compare them separately.
        """
        reader = fasta.UniProtReader(self.__class__.TEST_READ_FASTA_FILE, 1024)

        entries = [
            entry for entry in reader 
        ]

        writer = fasta.UniProtWriter(self.__class__.TEST_WRITE_FASTA_FILE)

        for entry in entries:
            writer.write_entry(entry)

        writer.flush()

        del writer

        with self.__class__.TEST_READ_FASTA_FILE.open("r") as in_file:
            with self.__class__.TEST_WRITE_FASTA_FILE.open("r") as out_file:
                for in_line, out_line in zip(in_file, out_file):
                    if not in_line.startswith(">"):
                        self.assertEqual(in_line, out_line)
                    else:
                        # find the first equals sign
                        first_equals_sign_pos: int = in_line.find("=")
                        # find the preceding whitespace (up to this point, the header should contains everything but the keyword attributes)
                        first_whitespace_pos: int = in_line.rfind(" ", 0, first_equals_sign_pos)
                        # compare the first part of the header
                        self.assertEqual(in_line[:first_whitespace_pos], out_line[:first_whitespace_pos])

                        # split them
                        in_keyword_attributes = self.__split_keyword_attributes(in_line[first_whitespace_pos:])
                        out_keyword_attributes = self.__split_keyword_attributes(out_line[first_whitespace_pos:])

                        # compare length
                        self.assertEqual(len(in_keyword_attributes), len(out_keyword_attributes))

                        for out_keyword_attribute in out_keyword_attributes:
                            self.assertIn(out_keyword_attribute, in_keyword_attributes)


        self.__class__.TEST_WRITE_FASTA_FILE.unlink(missing_ok=True)

        with self.assertRaises(RuntimeError):
            fasta.UniProtReader(self.__class__.TEST_NON_EXISTING_FASTA_FILE, 1024)

