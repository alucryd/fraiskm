<script lang="ts">
  import { goto } from "$app/navigation";
  import { faAt, faKey } from "@fortawesome/free-solid-svg-icons";
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

  import { login } from "../mutation.js";

  let username = "";
  let password = "";
  let errorMessage = "";

  $: usernameInvalid = errorMessage == "unknown username";
  $: passwordInvalid = errorMessage == "invalid password";

  async function onSubmit(event) {
    event.preventDefault();
    try {
      await login(username, password);
      goto("/", { replaceState: false });
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  }
</script>

<Row>
  <Col class="d-inline-flex">
    <Card class="mx-auto text-center">
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
            <Button block color="dark" type="submit">Valider</Button>
          </FormGroup>
        </Form>
      </CardBody>
      <CardFooter>
        <a href="/">Mot de passe oublié</a>
      </CardFooter>
    </Card>
  </Col>
</Row>
