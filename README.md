# Paul

Discord-Interaction-Handler für `/agent` und `/waffe`, welche einen zufälligen Agenten bzw. eine zufällige Waffe für Valorant vorschlagen.

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
curl https://discord.com/api/oauth2/token \
    -d client_id=<client_id> \
    -d client_secret=<client_secret> \
    -d grant_type=client_credentials \
    -d scope=application.commands.update
```

Folgender Befehl fügt den Slash-Command dem Client hinzu wobei der Body das JSON-Objekt des Slash-Commands ist.

```shell
curl https://discord.com/api/v10/applications/<client_id>/commands \
    -H "Authorization: Bearer <token>" \
    -H "Content-Type: application/json" \
    -d '<befehl>'
```

Alternativ auch mit einer Datei statt des Befehls direkt.

```shell
curl https://discord.com/api/v10/applications/<client_id>/commands \
    -H "Authorization: Bearer <token>" \
    -H "Content-Type: application/json" \
    -d @file
```
