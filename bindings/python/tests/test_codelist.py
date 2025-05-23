import unittest
from codelists_rs.codelist import CodeList


class TestCodeListBasics(unittest.TestCase):

    def setUp(self):
        self.codelist = CodeList(
            name="Test Codelist",
            codelist_type="ICD10",
            source="Manually created",
        )

    def test_create_basic_codelist(self):
        codelist = CodeList(
            name="Test Codelist",
            codelist_type="ICD10",
            source="Manually created",
        )
        codelist.add_entry("A00", "Cholera")
        self.assertEqual(codelist.entries(), [("A00", "Cholera")])

    def test_invalid_codelist_type(self):
        with self.assertRaises(ValueError) as e:
            CodeList(
                name="Test Codelist",
                codelist_type="INVALID",
                source="Manually created",
            )
        self.assertEqual(str(e.exception), "Invalid codelist type: INVALID")

    def test_add_duplicate_entries(self):
        self.codelist.add_entry("A01", "Typhoid fever")
        self.codelist.add_entry("A01", "Typhoid fever")  # duplicate entry
        self.assertEqual(len(self.codelist.entries()), 1)

    def test_contributors_and_dates(self):
        self.assertEqual(set(), self.codelist.contributors)
        self.codelist.add_contributor("John Doe")
        self.assertEqual({"John Doe"}, self.codelist.contributors)
        self.codelist.add_contributor("Jane Smith")
        self.assertEqual({"John Doe", "Jane Smith"}, self.codelist.contributors)
        self.codelist.remove_contributor("John Doe")
        self.assertEqual({"Jane Smith"}, self.codelist.contributors)
        self.assertIn("date_created", self.codelist.get_dates())
        self.assertIn("last_modified_date", self.codelist.get_dates())

    def test_tags_and_usage(self):
        self.codelist.add_tag("pneumonia")
        self.codelist.add_tag("fever")
        self.assertIn("pneumonia", self.codelist.get_tags())
        self.assertIn("fever", self.codelist.get_tags())
        self.codelist.remove_tag("fever")
        self.assertNotIn("fever", self.codelist.get_tags())

        self.codelist.add_usage("clinical trials")
        self.codelist.add_usage("research")
        self.assertIn("clinical trials", self.codelist.get_usage())
        self.assertIn("research", self.codelist.get_usage())
        self.codelist.remove_usage("research")
        self.assertNotIn("research", self.codelist.get_usage())

    def test_license_management(self):
        self.codelist.add_license("MIT")
        self.assertEqual("MIT", self.codelist.get_license_info())
        self.codelist.update_license("Apache 2.0")
        self.assertEqual("Apache 2.0", self.codelist.get_license_info())
        self.codelist.remove_license()
        self.assertIsNone(self.codelist.get_license_info())

    def test_purpose_and_context(self):
        self.codelist.add_purpose("To identify those with pneumonia in primary care")
        self.assertEqual("To identify those with pneumonia in primary care", self.codelist.get_purpose())
        self.codelist.update_purpose("To identify those with pneumonia in secondary care")
        self.assertEqual("To identify those with pneumonia in secondary care", self.codelist.get_purpose())
        self.codelist.remove_purpose()
        self.assertIsNone(self.codelist.get_purpose())

        self.codelist.add_audience("epidemiologists using CPRD")
        self.codelist.add_use_context("CPRD database")
        self.assertEqual("epidemiologists using CPRD", self.codelist.get_audience())
        self.assertEqual("CPRD database", self.codelist.get_use_context())
        self.codelist.update_audience("epidemiologists using CPRD and other databases")
        self.codelist.update_use_context("CPRD and other databases")
        self.assertEqual("epidemiologists using CPRD and other databases", self.codelist.get_audience())
        self.assertEqual("CPRD and other databases", self.codelist.get_use_context())
        self.codelist.remove_audience()
        self.codelist.remove_use_context()
        self.assertIsNone(self.codelist.get_audience())
        self.assertIsNone(self.codelist.get_use_context())

    def test_validation(self):
        self.assertEqual(False, self.codelist.is_validated())
        self.codelist.add_validation_info(
            reviewer="Michelle Obama",
            status="in review",
            notes="Looking good"
        )
        self.assertEqual("Michelle Obama", self.codelist.get_reviewer())
        self.assertEqual("in review", self.codelist.get_validation_status())
        self.assertEqual("Looking good", self.codelist.get_validation_notes())
        self.codelist.update_validation_notes("Needs more work")
        self.assertEqual("Looking good\nNeeds more work", self.codelist.get_validation_notes())


class TestCodeListValidation(unittest.TestCase):

    def test_validate_icd10(self):
        codelist = CodeList(
            name="Test Codelist",
            codelist_type="ICD10",
            source="Manually created",
        )
        codelist.add_entry("A02", "Salmonella infections")
        codelist.validate_codes()
        codelist.add_entry("INVALID_CODE", "Invalid code")
        with self.assertRaises(ValueError) as e:
            codelist.validate_codes()
        self.assertIn("Code INVALID_CODE is an invalid length", str(e.exception))

    def test_validate_snomed(self):
        codelist = CodeList(
            name="Test SNOMED",
            codelist_type="SNOMED",
            source="Manually created",
        )
        codelist.add_entry("123456", "Valid SNOMED")
        codelist.add_entry("12345678", "Valid SNOMED")
        codelist.validate_codes()
        codelist.add_entry("11", "Invalid SNOMED")
        with self.assertRaises(ValueError) as e:
            codelist.validate_codes()
        self.assertIn("Code 11 is an invalid length for type SNOMED", str(e.exception))

if __name__ == '__main__':
    unittest.main()
