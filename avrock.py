import avro.schema

from avro.datafile import DataFileReader, DataFileWriter
from avro.io import DatumReader, DatumWriter

schema = avro.schema.parse(open("schema2.json", "rb").read())
print(schema)

person_data = {
    "name": "John Doe",
    "address": {"street": "123 Main St", "city": "Anytown"},
    "phone_numbers": [
        {"street": "456 Elm St", "city": "Anotherville"},
        {"street": "789 Oak St", "city": "Sometown"},
    ],
}
writer = DataFileWriter(open("sample_data.avro", "wb"), DatumWriter(), schema)
writer.append(person_data)
writer.close()

# writer = DataFileWriter(open("users.avro", "wb"), DatumWriter(), schema)
# writer.append({"name": "Alyssa", "favorite_number": 256})
# writer.append({"name": "Ben", "favorite_number": 7, "favorite_color": "red"})
# writer.close()
#

print("-" * 80)
reader = DataFileReader(open("sample_data.avro", "rb"), DatumReader())
for user in reader:
    print(user)
reader.close()
