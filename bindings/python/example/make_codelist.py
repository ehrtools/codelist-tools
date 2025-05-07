from codelists_rs.codelist import CodeList
from codelists_rs.factory import CodeListFactory

# Create a new codelist object
c = CodeList("Pneumonia", "ICD10", "Manually created")

# Add a code to the list
c.add_entry("A119", "pneumonia")

# validate the codelist
c.validate_codes()

# Print the contents of the codelist
entries = c.entries()
print(f"Codelist entries for {c.name}:")
for code, label in entries:
    print(f"{code}: {label}")


# import copd csv
snomed_factory = CodeListFactory("snomed")
c = snomed_factory.load_from_file("COPD", "COPD.csv")
c.validate_codes()

print(f"\nCodelist entries for {c.name}:")
for code, label in c.entries():
    print(f"{code}: {label}")


