-- Add migration script here
CREATE TABLE devices (
  manufacturer varchar(128) not null,
  oui varchar(64) not null,
  product_class varchar(128) not null,
  serial_number varchar(128) not null,
  last_inform_at TIMESTAMPTZ,
  CONSTRAINT unique_device_identity unique (manufacturer, oui, product_class, serial_number)
);
