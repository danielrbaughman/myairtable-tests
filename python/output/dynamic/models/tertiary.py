# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from datetime import datetime, timedelta
from typing import Any, TYPE_CHECKING

from pyairtable.orm import Model
from pyairtable.orm.fields import SingleLineTextField, MultilineTextField, PhoneNumberField, EmailField, LinkField, SingleLinkField, UrlField, DateField, CreatedTimeField, LastModifiedTimeField, NumberField, SelectField, MultipleSelectField, CheckboxField, RichTextField, CurrencyField, PercentField, LookupField, AttachmentsField, CreatedByField, ButtonField, CountField, DatetimeField, DurationField, LastModifiedByField, AutoNumberField, CollaboratorField, MultipleCollaboratorsField

from ...static.helpers import get_api_key, get_base_id
from ...static.special_types import AirtableAttachment, RecordId
from ..dicts import TertiaryRecordDict
from ..formulas import TertiaryFormulas
if TYPE_CHECKING:
    from .secondary import SecondaryModel


class TertiaryModel(Model):
    """
    ORM model for Airtable records from the `Tertiary` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Tertiary"
        use_field_ids = True
        memoize = True

    def to_record_dict(self) -> TertiaryRecordDict:
        return self.to_record()

    f: TertiaryFormulas = TertiaryFormulas()

    name: SingleLineTextField = SingleLineTextField(field_name="fldwzqKxsRnPZJ2Ll")
    """Name `fldwzqKxsRnPZJ2Ll` - `Primary Key`"""
    secondary: list["SecondaryModel"] = LinkField["SecondaryModel"](field_name="fld8lCuUXpEXkIeYv", model="output.dynamic.models.secondary.SecondaryModel") # type: ignore
    """Secondary `fld8lCuUXpEXkIeYv`"""
    value: SingleLineTextField = SingleLineTextField(field_name="fldjNLBh2UccM64h5")
    """Value `fldjNLBh2UccM64h5`"""

