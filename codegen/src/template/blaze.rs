
pub fn get_template() -> String {
    r#"
import { Data } from "@blaze-cardano/sdk";

{{#each this}}

  {{~ #if (is_integer type_name) ~}}
const {{name}} = Data.Integer();
  {{/if}}

  {{~ #if (is_bytes type_name) ~}}
const {{name}} = Data.Bytes();
  {{/if}}

  {{~ #if (is_literal type_name) ~}}
const {{name}} = Data.Literal("{{properties.[0].schema_name}}");
  {{/if}}

  {{~ #if (is_nullable type_name) ~}}
const {{name}} = Data.Nullable({{properties.[0].schema_name}});
  {{/if}}

  {{~ #if (is_object type_name) ~}}
const {{name}} = Data.Object({
  {{#each properties}}
  {{name}}: {{schema_name}},
  {{/each}}
});
  {{/if}}

  {{~ #if (is_enum type_name) ~}}
const {{name}} = Data.Enum([
  {{#each properties}}
  {{schema_name}},
  {{/each}}
]);
  {{/if}}

  {{~ #if (is_tuple type_name) ~}}
const {{name}} = Data.Tuple([
  {{#each properties}}
  {{schema_name}},
  {{/each}}
]);
  {{/if}}

  {{~ #if (is_list type_name) ~}}
const {{name}} = Data.Array({{properties.[0].schema_name}});
  {{/if}}

{{/each}}
    "#.to_string()
}