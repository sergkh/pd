# Password derivation tool PD

Allows to derive a new password from a master password and an optional domain/service name.

## Usage

Example usage to get domain specific generated password for a site:

```bash
 $ pd -d test.com | pbcopy
Password: ***
```