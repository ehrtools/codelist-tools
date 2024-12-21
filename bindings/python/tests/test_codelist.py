
import unittest
from codelist import CodeList

class TestCodeList(unittest.TestCase):

    def test_create_basic_codelist(self):
        codelist = CodeList(
            codelist_type="ICD10",
            source="test",
        )
        codelist.add_entry("A00", "Cholera")
        self.assertEqual(codelist.entries(), [("A00", "Cholera")])


    def test_invalid_codelist_type(self):
        with self.assertRaises(ValueError) as e:
            CodeList(
                codelist_type="INVALID",
                source="test"
            )
        self.assertEqual(str(e.exception), "Invalid codelist type: INVALID")


    def test_add_duplicate_entries(self):
        codelist = CodeList(
            codelist_type="ICD10",
            source="test",
        )

        codelist.add_entry("A01", "Typhoid fever")
        codelist.add_entry("A01", "Typhoid fever")  # duplicate entry

        entries = codelist.entries()
        self.assertEqual(len(entries), 1)

if __name__ == '__main__':
    unittest.main()