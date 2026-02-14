# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from pyairtable import Table

from ...static.airtable_table import AirtableTable
from ..types import (
    PrimaryField,
    PrimaryCalculatedFields,
    PrimaryCalculatedFieldIds,
    PrimaryView,
    PrimaryViewNameIdMapping,
    PrimaryFields,
)
from ..dicts import (
    PrimaryRecordDict,
    PrimaryCreateRecordDict,
    PrimaryUpdateRecordDict,
)
from ..models import PrimaryModel
# endregion



# region PRIMARY
class PrimaryTable(AirtableTable[PrimaryRecordDict, PrimaryCreateRecordDict, PrimaryUpdateRecordDict, PrimaryModel, PrimaryView, PrimaryField]):
    """
    An abstraction of pyAirtable's `Api.table` for the `Primary` table, and an interface for working with custom-typed versions of the models/dicts created by the type generator.

    ```python
    record = Airtable().primary.get("rec1234567890")
    ```

    You can also access the RecordDicts via `.dict`.
    
    ```python
    record = Airtable().primary.dict.get("rec1234567890")
    ```

    You can also use the ORM Models directly. See https://pyairtable.readthedocs.io/en/stable/orm.html#
    """
    @classmethod
    def from_table(cls, table: Table):
        cls = super().from_table(
            table,
            PrimaryRecordDict,
            PrimaryCreateRecordDict,
            PrimaryUpdateRecordDict,
            PrimaryModel,
            PrimaryCalculatedFields,
            PrimaryCalculatedFieldIds,
            PrimaryViewNameIdMapping,
            PrimaryFields,
        )
        return cls
# endregion


