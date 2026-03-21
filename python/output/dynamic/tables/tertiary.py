# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from pyairtable import Table

from ...static.airtable_table import AirtableTable
from ..types import (
    TertiaryField,
    TertiaryCalculatedFields,
    TertiaryCalculatedFieldIds,
    TertiaryView,
    TertiaryViewNameIdMapping,
    TertiaryFields,
)
from ..dicts import (
    TertiaryRecordDict,
    TertiaryCreateRecordDict,
    TertiaryUpdateRecordDict,
)
from ..models import TertiaryModel
# endregion



# region TERTIARY
class TertiaryTable(AirtableTable[TertiaryRecordDict, TertiaryCreateRecordDict, TertiaryUpdateRecordDict, TertiaryModel, TertiaryView, TertiaryField]):
    """
    An abstraction of pyAirtable's `Api.table` for the `Tertiary` table, and an interface for working with custom-typed versions of the models/dicts created by the type generator.

    ```python
    record = Airtable().tertiary.get("rec1234567890")
    ```

    You can also access the RecordDicts via `.dict`.
    
    ```python
    record = Airtable().tertiary.dict.get("rec1234567890")
    ```

    You can also use the ORM Models directly. See https://pyairtable.readthedocs.io/en/stable/orm.html#
    """
    @classmethod
    def from_table(cls, table: Table, cache_seconds: int = 0):
        cls = super().from_table(
            table,
            TertiaryRecordDict,
            TertiaryCreateRecordDict,
            TertiaryUpdateRecordDict,
            TertiaryModel,
            TertiaryCalculatedFields,
            TertiaryCalculatedFieldIds,
            TertiaryViewNameIdMapping,
            TertiaryFields,
            cache_seconds=cache_seconds,
        )
        return cls
# endregion


