import { Navbar } from "./Navbar";
import { useAuth } from "./utils/auth";

const Home = () => {
  useAuth();

  return (
    <>
      <Navbar />
    </>
  );
};

export default Home;
