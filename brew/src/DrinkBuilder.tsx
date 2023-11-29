import { useMutation, useQuery } from "@apollo/client";
import { maybe } from "@tsly/maybe";
import { maybeParseInt } from "@tsly/core";
import { Box, Button, MultiSelect, Select, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect } from "react";
import { graphql } from "@/gql";
import {
  addonsQuery,
  basesQuery,
  createDrinkMutation,
  tempsQuery,
  updateDrinkMutation,
} from "@/utils/gql";

const getDrinkByIdQuery = graphql(`
  query GetDrinkById($id: Int!) {
    drink(id: $id) {
      name
      temperature {
        id
      }
      base {
        id
      }
      addons {
        id
      }
    }
  }
`);

type DrinkBuilderProps = { id: "new" | number; onDone: () => void };

export const DrinkBuilder = (props: DrinkBuilderProps) => {
  const form = useForm({
    initialValues: {
      name: "",
      temperatureId: null as null | number,
      baseId: null as null | number,
      addonIds: [] as number[],
    },
  });

  const [addDrink, { data: addData, loading: addIsLoading }] =
    useMutation(createDrinkMutation);
  const [updateDrink, { data: updateData, loading: updateIsLoading }] =
    useMutation(updateDrinkMutation);

  const { data: tempsData } = useQuery(tempsQuery);
  const { data: basesData } = useQuery(basesQuery);
  const { data: addonsData } = useQuery(addonsQuery);

  const { data: drinkData } = useQuery(getDrinkByIdQuery, {
    variables: { id: maybeParseInt(props.id.toString()) ?? -1 },
    fetchPolicy: "no-cache",
  });

  useEffect(() => {
    if (
      addData?.addDrink.id != undefined ||
      updateData?.updateDrink.id != undefined
    ) {
      props.onDone();
    }
  }, [addData, updateData]);

  useEffect(() => {
    const drink = drinkData?.drink;

    if (drink) {
      form.setFieldValue("name", drink.name);
      form.setFieldValue("baseId", drink.base.id);
      form.setFieldValue("temperatureId", drink.temperature.id);
      form.setFieldValue(
        "addonIds",
        drink.addons.map((each) => each.id),
      );
    }
  }, [drinkData]);

  return (
    <Box maw={340} mx="auto" display={"flex"} className="flex-col gap-2">
      <TextInput
        label="Drink Name"
        placeholder="Name..."
        {...form.getInputProps("name")}
      />

      <Select
        label="Drink Base"
        placeholder="Select..."
        searchable
        value={form.values.baseId?.toString()}
        data={
          basesData?.bases.map((temp) => ({
            label: temp.name,
            value: temp.id.toString(),
          })) ?? []
        }
        onChange={(id) =>
          form.setFieldValue(
            "baseId",
            maybe(id)?.let(maybeParseInt)?.take() ?? null,
          )
        }
      />

      <Select
        label="Temperature"
        placeholder="Select..."
        searchable
        value={form.values.temperatureId?.toString()}
        data={
          tempsData?.temperatures.map((temp) => ({
            label: temp.name,
            value: temp.id.toString(),
          })) ?? []
        }
        onChange={(id) =>
          form.setFieldValue(
            "temperatureId",
            maybe(id)?.let(maybeParseInt)?.take() ?? null,
          )
        }
      />

      <MultiSelect
        label="Addons"
        searchable
        data={
          addonsData?.addons.map((addon) => ({
            label: addon.name,
            value: addon.id.toString(),
          })) ?? []
        }
        value={form.values.addonIds.map((id) => id.toString())}
        onChange={(vals) =>
          form.setFieldValue(
            "addonIds",
            vals.map((each) => Number.parseInt(each)),
          )
        }
      />

      <Button
        loading={addIsLoading || updateIsLoading}
        className="w-fit mt-2 self-end"
        onClick={() => {
          const { addonIds, baseId, name, temperatureId } = form.values;
          if (baseId != null && temperatureId != null && name != "") {
            if (props.id == "new")
              addDrink({
                variables: {
                  drink: {
                    name,
                    addonIds,
                    baseId,
                    temperatureId,
                  },
                },
              });
            else
              updateDrink({
                variables: {
                  id: props.id,
                  update: {
                    name,
                    addonIds,
                    baseId,
                    temperatureId,
                  },
                },
              });
          }
        }}
        disabled={
          addIsLoading ||
          updateIsLoading ||
          form.values.baseId == null ||
          form.values.temperatureId == null ||
          form.values.name.trim() == ""
        }
      >
        {props.id == "new" ? "Create" : "Save"}
      </Button>
    </Box>
  );
};
