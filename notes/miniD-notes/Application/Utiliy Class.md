# Methods naming conventions
## Parse / Deserialize / Decode
Used when **input is untrusted or raw** (JSON, text, external API)
“Turn raw data into structured, typed objects”
- `parse(...)`
- `parseMedia(...)`
- `deserialize(...)`
- `fromJson(...)`
- `decode(...)`
## Create / Build / Make
Used when **constructing new domain objects**
“Produce a valid object from known inputs”
- `createMedia(...)`
- `buildMedia(...)`
- `makeVideoMedia(...)`
## **Normalize / Sanitize / Transform**
Used when **data is mostly valid but inconsistent**
“Clean this data into a standard shape”
- `normalize(...)`
- `sanitizeFormats(...)`
- `transformMedia(...)`
## **Resolve / Infer / Detect**
Used when **decisions are made from data**
“Figure out what this thing actually is”
- `resolveMediaType(...)`
- `inferAudioOnly(...)`
- `detectFormat(...)`
## Filter / Select / Extract
Used when **reducing or choosing subsets**
“Give me only what I care about”
- `filterDownloadable(...)`
- `selectBestFormat(...)`
- `extractAudioFormats(...)`
## Load / Fetch / Request
Used when **I/O or side effects occur**
“Get data from somewhere else”