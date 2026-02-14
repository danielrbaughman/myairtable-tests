# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from typing import Any

from pyairtable.api.types import CreateRecordDict, RecordDict, UpdateRecordDict

from ..types import (
    TertiaryFieldsDict,
    TertiaryField,
)

class TertiaryCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: dict[TertiaryField, Any]


class TertiaryIdsCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: TertiaryFieldsDict


class TertiaryUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: dict[TertiaryField, Any]


class TertiaryIdsUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: TertiaryFieldsDict


class TertiaryRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: dict[TertiaryField, Any]


class TertiaryIdsRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Tertiary` table.

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
    fields: TertiaryFieldsDict


