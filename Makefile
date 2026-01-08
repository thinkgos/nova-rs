gen.model:
	@sea generate entity \
	 -u ${DATABASE_URL} \
	 --entity-format dense \
	 --with-serde both \
	 --model-extra-derives "utoipa::ToSchema" \
	 --model-extra-attributes 'serde(rename_all = "camelCase")' \
	 --ignore-tables atlas_schema_revisions \
	 -o crates/dal/src/model

.PHONY: gen.model