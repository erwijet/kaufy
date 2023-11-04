import { useAuth } from "./utils/auth";
import { useQuery, gql } from "@apollo/client";

function App() {
  useAuth();

  const { data } = useQuery(
    gql`
      query {
        drinks {
          name,
          owner {
            givenName,
            familyName,
            picture
          }
        }
      }
    `,
  );

  return <ul>
    {/* {data.map(drink => {

    })} */}
  </ul>

  return <pre>{JSON.stringify({ data }, null, 2)}</pre>;
}

export default App;
