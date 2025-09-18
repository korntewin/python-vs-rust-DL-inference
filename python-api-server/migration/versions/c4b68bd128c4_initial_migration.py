"""initial migration

Revision ID: c4b68bd128c4
Revises:
Create Date: 2025-09-13 10:58:00.706260

"""

from typing import Sequence, Union

import sqlalchemy as sa
from alembic import op
from geoalchemy2 import Geography

# revision identifiers, used by Alembic.
revision: str = "c4b68bd128c4"
down_revision: Union[str, Sequence[str], None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    """Upgrade schema."""
    op.execute("CREATE EXTENSION IF NOT EXISTS postgis;")

    op.create_table(
        "feature_1",
        sa.Column("id", sa.BigInteger(), primary_key=True),
        sa.Column("feature", sa.ARRAY(sa.Float()), nullable=False),
    )

    op.create_table(
        "feature_2",
        sa.Column("id", sa.BigInteger(), primary_key=True),
        sa.Column("geog", Geography(geometry_type="POINT", srid=4326), nullable=False),
        sa.Column("feature", sa.ARRAY(sa.Float()), nullable=False),
    )


def downgrade() -> None:
    """Downgrade schema."""
    op.drop_table("feature_2")
    op.drop_table("feature_1")
    op.execute("DROP EXTENSION IF EXISTS postgis;")
