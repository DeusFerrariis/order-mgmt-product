package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
)

// Product holds the schema definition for the Product entity.
type Product struct {
	ent.Schema
}

// Fields of the Product.
func (Product) Fields() []ent.Field {
	return []ent.Field{
		// Product SKU
		field.String("sku").
			NotEmpty(),
		// Product Description
		field.String("description").
			NotEmpty(),
		// Product Vendor Cost
		field.Int("cost").
			Positive().
			Min(1),
		// Product Price
		field.Int("price").
			Positive().
			Min(1),
	}
}

// Edges of the Product.
func (Product) Edges() []ent.Edge {
	return nil
}
