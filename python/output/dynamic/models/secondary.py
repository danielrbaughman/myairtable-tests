# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from datetime import datetime, timedelta
from typing import Any, TYPE_CHECKING

from pyairtable.orm import Model
from pyairtable.orm.fields import SingleLineTextField, MultilineTextField, PhoneNumberField, EmailField, LinkField, SingleLinkField, UrlField, DateField, CreatedTimeField, LastModifiedTimeField, NumberField, SelectField, MultipleSelectField, CheckboxField, RichTextField, CurrencyField, PercentField, LookupField, AttachmentsField, CreatedByField, ButtonField, CountField, DatetimeField, DurationField, LastModifiedByField, AutoNumberField, CollaboratorField, MultipleCollaboratorsField

from ...static.helpers import get_api_key, get_base_id
from ...static.special_types import AirtableAttachment, RecordId
from ..dicts import SecondaryRecordDict
from ..formulas import SecondaryFormulas
if TYPE_CHECKING:
    from .tertiary import TertiaryModel
    from .primary import PrimaryModel


class SecondaryModel(Model):
    """
    ORM model for Airtable records from the `Secondary` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Secondary"
        use_field_ids = True
        memoize = True

    def to_record_dict(self) -> SecondaryRecordDict:
        return self.to_record()

    f: SecondaryFormulas = SecondaryFormulas()

    evaluate_formulas_at_runtime: bool = False

    link_to_tertiary: list["TertiaryModel"] = LinkField["TertiaryModel"](field_name="fldKR6tdbnOBRCtdQ", model="output.dynamic.models.tertiary.TertiaryModel") # type: ignore
    """Link to Tertiary `fldKR6tdbnOBRCtdQ`"""
    name: SingleLineTextField = SingleLineTextField(field_name="fld1RagdJ09mpWhzM")
    """Name `fld1RagdJ09mpWhzM` - `Primary Key`"""
    primary: list["PrimaryModel"] = LinkField["PrimaryModel"](field_name="fldl0nB9WRFSdqlii", model="output.dynamic.models.primary.PrimaryModel") # type: ignore
    """Primary `fldl0nB9WRFSdqlii`"""
    primary_2: list["PrimaryModel"] = LinkField["PrimaryModel"](field_name="fldgoE2oZmXmKkQca", model="output.dynamic.models.primary.PrimaryModel") # type: ignore
    """Primary 2 `fldgoE2oZmXmKkQca`"""
    value: SingleLineTextField = SingleLineTextField(field_name="fldi6Mxh5H1gPGxFX")
    """Value `fldi6Mxh5H1gPGxFX`"""

