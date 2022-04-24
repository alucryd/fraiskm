<script lang="ts">
  import { goto } from "$app/navigation";
  import { faArrowRight, faAt, faKey } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import {
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardTitle,
    Col,
    Form,
    FormGroup,
    Input,
    InputGroup,
    InputGroupText,
    Row,
  } from "sveltestrap";

  import Transition from "../components/Transition.svelte";
  import { signin } from "../mutation.js";
  import { getAddresses, getDrivers, getVehicles } from "../query.js";
  import { drivers, journeyDriverId, ready } from "../store";

  let username = "";
  let password = "";

  let errorMessage = "";

  $: usernameInvalid = errorMessage == "unknown username";
  $: passwordInvalid = errorMessage == "invalid password";

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      await signin(username, password);
      await getVehicles();
      await getAddresses();
      await getDrivers();
      if ($drivers.length) {
        $journeyDriverId = $drivers[0].id;
      }
      $ready = true;
      goto("/", { replaceState: false });
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Transition>
  <Row class="justify-content-center">
    <Col class="col-lg-4 col-md-6 col-sm-12">
      <Card class="text-center">
        <CardHeader>
          <CardTitle class="mb-0">Connexion</CardTitle>
        </CardHeader>
        <CardBody class="pb-0">
          <Form on:submit={onSubmit}>
            <FormGroup>
              <InputGroup>
                <InputGroupText>
                  <Fa icon={faAt} />
                </InputGroupText>
                <Input
                  type="email"
                  name="email"
                  id="email"
                  placeholder="Adresse email"
                  feedback="L'adresse email est inconnue"
                  bind:invalid={usernameInvalid}
                  bind:value={username}
                  required
                />
              </InputGroup>
            </FormGroup>
            <FormGroup>
              <InputGroup>
                <InputGroupText>
                  <Fa icon={faKey} />
                </InputGroupText>
                <Input
                  type="password"
                  name="password"
                  id="password"
                  placeholder="Mot de passe"
                  feedback="Le mot de passe est invalide"
                  bind:invalid={passwordInvalid}
                  bind:value={password}
                  required
                />
              </InputGroup>
            </FormGroup>
            <FormGroup>
              <Button block color="dark" type="submit" disabled={!username || !password}>Valider</Button>
            </FormGroup>
          </Form>
        </CardBody>
        <CardFooter>
          <Button block color="dark" href="/signup">
            Enregistrement
            <Fa icon={faArrowRight} />
          </Button>
        </CardFooter>
      </Card>
    </Col>
  </Row>
</Transition>
