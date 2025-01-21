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

    def test_metadata(self):
        metadata = {
            "provenance": {
                "source": "MANUAL",  # This maps to Provenance
            },
            "categorisation_and_usage": {
                "authors": ["John Doe", "Jane Smith"],
                "keywords": ["test", "example"],
            },
            "purpose_and_context": {
                "version": "1.0",
                "purpose": "Testing metadata functionality",
            },
            "validation_and_review": {
                "description": "Test codelist with metadata",
                "review_status": "DRAFT",
            }
        }
        
        codelist = CodeList(
            codelist_type="ICD10",
            source="test",
            metadata=metadata
        )
        
        # Test metadata retrieval
        self.assertEqual(codelist.get_authors(), ["John Doe", "Jane Smith"])
        self.assertEqual(codelist.get_version(), "1.0")
        self.assertEqual(codelist.get_description(), "Test codelist with metadata")
        self.assertEqual(codelist.get_review_status(), "DRAFT")

    def test_invalid_metadata(self):
        invalid_metadata = {
            "provenance": {
                "source": "INVALID_SOURCE",  # Invalid source type
            }
        }
        
        with self.assertRaises(ValueError) as e:
            CodeList(
                codelist_type="ICD10",
                source="test",
                metadata=invalid_metadata
            )
        self.assertIn("Invalid provenance source", str(e.exception))

if __name__ == '__main__':
    unittest.main()