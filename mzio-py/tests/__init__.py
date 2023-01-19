from typing import ClassVar
import unittest

from pathlib import Path

import rusteomics_proteomics_io_py


class FastaModuleTestCase(unittest.TestCase):
    EXPECTED_NUM_OF_PROTEINS: ClassVar[int] = 37243;

    def test_reader(self):
        reader = rusteomics_proteomics_io_py.fasta.Reader(Path("../test_files/fasta/mouse.fasta"), 1024)
        accessions: Set[str] = set()
        prot_ctr: int = 0
        for prot in reader:
            accessions.add(prot.accession)
            prot_ctr += 1

        self.assertEqual(prot_ctr, self.__class__.EXPECTED_NUM_OF_PROTEINS) # Check if the reader does read exactly the number of expected proteins
        self.assertEqual(len(accessions), self.__class__.EXPECTED_NUM_OF_PROTEINS) # Check if every protein is read
