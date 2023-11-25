import { graphql } from "@/gql";

export const tempsQuery = graphql(`
  query temperatures {
    temperatures {
      id
      name
    }
  }
`);

export const basesQuery = graphql(`
  query bases {
    bases {
      id
      name
    }
  }
`);

export const addonsQuery = graphql(`
  query addons {
    addons {
      id
      name
    }
  }
`);

export const createDrinkMutation = graphql(`
  mutation CreateDrink($drink: DrinkInputObject!) {
    addDrink(input: $drink) {
      id
    }
  }
`);

export const updateDrinkMutation = graphql(`
  mutation UpdateDrink($id: Int!, $update: DrinkInputObject!) {
    updateDrink(id: $id, update: $update) {
      id
    }
  }
`);
