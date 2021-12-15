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

  let username = "";
  let password = "";
  let errorMessage = "";

  $: usernameInvalid = errorMessage == "unknown username";
  $: passwordInvalid = errorMessage == "invalid password";

  async function onSubmit(event) {
    event.preventDefault();
    try {
      await signin(username, password);
      goto("/", { replaceState: false });
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  }
</script>

<Transition>
  <Row>
    <Col class="d-inline-flex">
      <Card class="mx-auto text-center" style="min-width: 33em">
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
                  feedback="Adresse email inconnue"
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
                  feedback="Mot de passe invalide"
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
