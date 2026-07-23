# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from ...static.formula import (
    AttachmentsField,
    BooleanField,
    DateField,
    LookupField,
    NumberField,
    TextField,
    SingleSelectField,
    MultiSelectField,
    ID,
)
from ..types import (
    PrimaryMultipleSelectOption,
    PrimarySingleSelectOption,
)

# region PROPERTIES
class PrimaryFormulas:
    id: ID = ID()
    attachment: AttachmentsField = AttachmentsField('fldhF2AEuSC1haCZd')
    """
    Attachment `fldhF2AEuSC1haCZd`
    
    Closes early */ then more text
    """
    auto_number: NumberField = NumberField('fldizvTkxgIn0mC3L')
    """
    Auto Number `fldizvTkxgIn0mC3L` - `Read-Only`
    
    Opens a nested /* comment
    """
    button: TextField = TextField('fldY48yKPG16AajtU')
    """
    Button `fldY48yKPG16AajtU` - `Read-Only`
    
    Has a \"\"\" triple quote inside
    """
    checkbox: BooleanField = BooleanField('fldjQIaAZVegb1FUa')
    """
    Checkbox `fldjQIaAZVegb1FUa`
    
    Ends with a backslash \\
    """
    created_by: TextField = TextField('fldGLQhDz2UjjiHG6')
    """
    Created By `fldGLQhDz2UjjiHG6` - `Read-Only`
    
    Value must be < 10 & > 0
    """
    created_at_time: DateField = DateField('fld2YgW382Kt9xltA')
    """
    Created Time `fld2YgW382Kt9xltA` - `Read-Only`
    
    Literal </summary> tag here
    """
    currency_float: NumberField = NumberField('fldyh8pzDXiy5abEr')
    """
    Currency (float) `fldyh8pzDXiy5abEr`
    
    Mixed "double" and 'single' quotes
    """
    currency_int: NumberField = NumberField('fldBfo74z9hD78hP8')
    """
    Currency (int) `fldBfo74z9hD78hP8`
    
    leading and trailing spaces (and one that is only whitespace)
    """
    date: DateField = DateField('fldC6LfNVvVIxKyQH')
    """
    Date `fldC6LfNVvVIxKyQH`
    
    A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)A very long single-line description (200+ chars, no newlines)
    """
    date_with_time: DateField = DateField('fldizYmjpXABGDLTG')
    """
    Date (with time) `fldizYmjpXABGDLTG`
    
    Use @param and @return in prose
    """
    duplicate_name: TextField = TextField('fld99XdcwRa5WW6nw')
    """
    Duplicate (Name) `fld99XdcwRa5WW6nw`
    
    Doxygen \\brief and \\code words
    """
    duplicate_name_v2: TextField = TextField('fld95OnbzAUFUTTqZ')
    """
    Duplicate Name `fld95OnbzAUFUTTqZ`
    
    Refer to the ``code`` field (backticks)
    """
    duration: NumberField = NumberField('fldLTyf6ljS0rhur8')
    """
    Duration `fldLTyf6ljS0rhur8`
    
    Café ☕ 日本語 — em-dash (Unicode + emoji + em-dash)
    """
    email: TextField = TextField('fldHCJoYBiFVsNvP4')
    """
    Email `fldHCJoYBiFVsNvP4`
    
    A plain writable field with a simple one-line description
    """
    formula_complex: TextField = TextField('fld2vnFc0Bl5IOFUQ')
    """
    Formula (Complex) `fld2vnFc0Bl5IOFUQ` - `Read-Only`
    
    Concatenates all field values into a readable summary, using correct field references.
    
    ```
    CONCATENATE(
      "Primary Key: ",
      {Primary Key},
      "\\n",
      "Single Line Text: ",
      {Single Line Text},
      "\\n",
      "Long Text: ",
      {Long Text},
      "\\n",
      "Long Text with Rich Text: ",
      {Long Text with Rich Text},
      "\\n",
      "Attachment: ",
      IF(
    … (truncated)
    ```
    """
    formula_id: TextField = TextField('fldcf62YFeIIDHElt')
    """
    Formula (ID) `fldcf62YFeIIDHElt` - `Read-Only`
    
    A description on a formula field (so it has both a description and a formula block)
    
    ```
    RECORD_ID()
    ```
    """
    formula_nested: TextField = TextField('fldXFeHRPBLz6AiWh')
    """
    Formula (Nested) `fldXFeHRPBLz6AiWh` - `Read-Only`
    
    ```
    {Formula (ID)} & {Formula (Simple)} & {Formula (Complex)}
    ```
    """
    formula_simple: NumberField = NumberField('fldy1axxaoUToLVC6')
    """
    Formula (Simple) `fldy1axxaoUToLVC6` - `Read-Only`
    
    ```
    {Number (int)} + {Number (float)}
    ```
    """
    last_modified_by: TextField = TextField('fldF8iDttqP0AgzWC')
    """Last Modified By `fldF8iDttqP0AgzWC` - `Read-Only`"""
    last_modified_time: DateField = DateField('fldMinKh4pa3YX86g')
    """Last Modified Time `fldMinKh4pa3YX86g` - `Read-Only`"""
    link_multiple: TextField = TextField('fldFyFheQWczd8oux')
    """Link (multiple) `fldFyFheQWczd8oux`"""
    link_single: TextField = TextField('fld7F5onkDo6mkmbN')
    """Link (single) `fld7F5onkDo6mkmbN`"""
    long_text: TextField = TextField('fld8ulc6J0W29M6La')
    """
    Long Text `fld8ulc6J0W29M6La`
    
    Description with special characters !@#$%^&*()<>/:"{}[]\\|=/*-+.,.?`~
    """
    long_text_with_rich_text: TextField = TextField('fldHJkxCMC0xo343u')
    """Long Text with Rich Text `fldHJkxCMC0xo343u`"""
    lookup: LookupField = LookupField('fldbmFmrzYKBktJvE')
    """Lookup `fldbmFmrzYKBktJvE` - `Read-Only`"""
    multiple_select: MultiSelectField[PrimaryMultipleSelectOption] = MultiSelectField('fld6GTabFmu1xKPvZ')
    """Multiple Select `fld6GTabFmu1xKPvZ`"""
    number_float: NumberField = NumberField('fldmU0X2l4RWd21dd')
    """Number (float) `fldmU0X2l4RWd21dd`"""
    number_int: NumberField = NumberField('fldOfPKGmnRPv94QH')
    """Number (int) `fldOfPKGmnRPv94QH`"""
    percent_float: NumberField = NumberField('fldiGui9ll69N7WOj')
    """Percent (float) `fldiGui9ll69N7WOj`"""
    percent_int: NumberField = NumberField('fldbAAyWboGulpb4s')
    """Percent (int) `fldbAAyWboGulpb4s`"""
    phone_number: TextField = TextField('fld38tnNpHmoks8C8')
    """Phone Number `fld38tnNpHmoks8C8`"""
    primary_key: TextField = TextField('fldol5Q4wmQJQvPRy')
    """
    Primary Key `fldol5Q4wmQJQvPRy` - `Primary Key`
    
    A description on the primary field
    """
    rating: TextField = TextField('fldRsmwFwQNZkKLp4')
    """Rating `fldRsmwFwQNZkKLp4`"""
    rollup: TextField = TextField('fldGaFgBsDC3IBUdV')
    """
    Rollup `fldGaFgBsDC3IBUdV` - `Read-Only`
    
    A description on a computed/rollup field
    """
    single_line_text: TextField = TextField('fld0BL2lFo9fqcKv3')
    """
    Single Line Text `fld0BL2lFo9fqcKv3`
    
    Description
    With
    New
    Lines
    """
    single_select: SingleSelectField[PrimarySingleSelectOption] = SingleSelectField('fldn0GFFtMFpCXUNU')
    """Single Select `fldn0GFFtMFpCXUNU`"""
    url: TextField = TextField('fldLYloz2oP4ymf3B')
    """URL `fldLYloz2oP4ymf3B`"""
    user: TextField = TextField('fldU6SbLp8CSkLcA4')
    """User `fldU6SbLp8CSkLcA4`"""
    user_allow_multiple: TextField = TextField('fldBwCDbAVxRj9yg7')
    """User (allow multiple) `fldBwCDbAVxRj9yg7`"""

# endregion

