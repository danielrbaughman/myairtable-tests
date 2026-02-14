# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from typing import Any

from pyairtable.api.types import CreateRecordDict, RecordDict, UpdateRecordDict

from ..types import (
    FormulasFieldsDict,
    FormulasField,
)

class FormulasCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field names

    ```
    {
        			"fields": {
            "Name": "Alice",
            "Department": "Engineering"
        }
    }
    ```
    """
    fields: dict[FormulasField, Any]


class FormulasIdsCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field ids

    ```
    {
        			"fields": {
            "fld75gvKPpwKmG58B": "Alice",
            "fldrEdQBTxp1Y8kKL": "Engineering"
        }
    }
    ```
    """
    fields: FormulasFieldsDict


class FormulasUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field names

    ```
    {
        "id": "recAdw9EjV90xbW",
			"fields": {
            "Name": "Alice",
            "Department": "Engineering"
        }
    }
    ```
    """
    fields: dict[FormulasField, Any]


class FormulasIdsUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field ids

    ```
    {
        "id": "recAdw9EjV90xbW",
			"fields": {
            "fld75gvKPpwKmG58B": "Alice",
            "fldrEdQBTxp1Y8kKL": "Engineering"
        }
    }
    ```
    """
    fields: FormulasFieldsDict


class FormulasRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field names

    ```
    {
        "id": "recAdw9EjV90xbW",
"createdTime": "2023-05-22T21:24:15.333134Z",
			"fields": {
            "Name": "Alice",
            "Department": "Engineering"
        }
    }
    ```
    """
    fields: dict[FormulasField, Any]


class FormulasIdsRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Formulas` table.

    A type-hinted version of the pyairtable `RecordDict` class.

    `fields` are all Airtable field ids

    ```
    {
        "id": "recAdw9EjV90xbW",
			"fields": {
            "fld75gvKPpwKmG58B": "Alice",
            "fldrEdQBTxp1Y8kKL": "Engineering"
        }
    }
    ```
    """
    fields: FormulasFieldsDict


