# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from typing import Any
from pyairtable.api.types import (
    CreateRecordDict,
    RecordDict,
    UpdateRecordDict,
)
from ..types import (
    SecondaryFieldsDict,
    SecondaryField,
)

class SecondaryCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: dict[SecondaryField, Any]  # ty: ignore


class SecondaryIdsCreateRecordDict(CreateRecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: SecondaryFieldsDict  # ty: ignore


class SecondaryUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: dict[SecondaryField, Any]  # ty: ignore


class SecondaryIdsUpdateRecordDict(UpdateRecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: SecondaryFieldsDict  # ty: ignore


class SecondaryRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: dict[SecondaryField, Any]  # ty: ignore


class SecondaryIdsRecordDict(RecordDict):
    """
    TypedDict representation for Airtable records from the `Secondary` table.

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
    fields: SecondaryFieldsDict  # ty: ignore


