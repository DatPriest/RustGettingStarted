import requests, json
url = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query?"
parameter = {
    'where': 'AdmUnitId=4011', # Alle Status-Datensätze
    'outFields': '*', # Rückgabe aller Felder
    'returnGeometry': False, # Keine Geometrien
    'f':'json', # Rückgabeformat, hier JSON
}
result = requests.get(url=url, params=parameter) #Anfrage absetzen
url = result.url
print(url)
resultjson = json.loads(result.text) # Das Ergebnis JSON als Python Dictionary laden

print(resultjson)