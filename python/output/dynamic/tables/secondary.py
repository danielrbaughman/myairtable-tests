# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from pyairtable import Table

from ...static.airtable_table import AirtableTable
from ..types import (
    SecondaryField,
    SecondaryCalculatedFields,
    SecondaryCalculatedFieldIds,
    SecondaryView,
    SecondaryViewNameIdMapping,
    SecondaryFields,
)
from ..dicts import (
    SecondaryRecordDict,
    SecondaryCreateRecordDict,
    SecondaryUpdateRecordDict,
)
from ..models import SecondaryModel
# endregion



# region SECONDARY
class SecondaryTable(AirtableTable[SecondaryRecordDict, SecondaryCreateRecordDict, SecondaryUpdateRecordDict, SecondaryModel, SecondaryView, SecondaryField]):
    """
    An abstraction of pyAirtable's `Api.table` for the `Secondary` table, and an interface for working with custom-typed versions of the models/dicts created by the type generator.

    ```python
    record = Airtable().secondary.get("rec1234567890")
    ```

    You can also access the RecordDicts via `.dict`.
    
    ```python
    record = Airtable().secondary.dict.get("rec1234567890")
    ```

    You can also use the ORM Models directly. See https://pyairtable.readthedocs.io/en/stable/orm.html#
    """
    @classmethod
    def from_table(cls, table: Table):
        cls = super().from_table(
            table,
            SecondaryRecordDict,
            SecondaryCreateRecordDict,
            SecondaryUpdateRecordDict,
            SecondaryModel,
            SecondaryCalculatedFields,
            SecondaryCalculatedFieldIds,
            SecondaryViewNameIdMapping,
            SecondaryFields,
        )
        return cls
# endregion


