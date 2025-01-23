# Paul

Discord-Interaction-Handler für `/agent`, `/weapon` und `/armor`, welche einen zufälligen Agenten, eine zufällige Waffe oder ein zufälliges Rüstungsteil für Valorant vorschlagen.

## Befehl registrieren

Zuerst neue App im Discord-Dev-Portal registrieren. Installiert wird sie in einer Gilde mit beispielsweise: `https://discord.com/oauth2/authorize?client_id=<client_id>`.

Wichtige Variablen:

| Name          | Grund                                                                                 |
|---------------|---------------------------------------------------------------------------------------|
| Client ID     | Die ID des Clients. Damit werden viele Endpunkte angesprochen. Identifiziert die App. |
| Client Secret | Damit werden Token erstellt um Befehle hinzuzufügen                                   |
| Public Key    | Wichtig für die Verifizierung der Interaction-Requests.                               |

Mit folgendem Befehl wird ein Bearer-Token angefordert, welcher es erlaubt, Befehle hinzuzufügen und zu verwalten.

```shell
curl -X POST https://discord.com/api/oauth2/token \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d client_id=$CLIENT_ID \
    -d client_secret=$CLIENT_SECRET \
    -d grant_type=client_credentials \
    -d scope=applications.commands.update
```

Folgender Befehl fügt den Slash-Command dem Client hinzu wobei der Body das JSON-Objekt des Slash-Commands ist.

```shell
curl -X POST https://discord.com/api/v10/applications/$CLIENT_ID/commands \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d '<befehl>'
```

Alternativ auch mit einer Datei statt des Befehls direkt.

```shell
curl -X POST https://discord.com/api/v10/applications/$CLIENT_ID/commands \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d @file
```

## Todo

- Eventuell nicht alles hartkodieren, environment nutzen.
- Bilder nicht von [Valorant-API](https://valorant-api.com) klauen, sondern selbst machen. (?)
- Kein `include_bytes!` verwenden, sondern Bilder beim 1. mal laden und cachen.
- Git actions?
- Refactoring.
- Hoffen, dass ich keinen Token commited habe. (Habe ich - ist revoked.)
- Axum & Tokio sind sehr heavy, wenn ich schon auf serenity verzichte.
- Möglich, direkt in der response das image senden? Keine Ahnung. Never change a running system.
- Slash commands nicht manuell erstellen sondern via code.
- Mehr metadaten füllen, nicht nur das mindeste?
- Add a license and check others'
- Denglisch
