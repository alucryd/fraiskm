<script lang="ts">
  import { goto } from "$app/navigation";
  import { faArrowRight, faAt, faCheck, faKey, faTimes } from "@fortawesome/free-solid-svg-icons";
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
    ListGroup,
    ListGroupItem,
    Row,
  } from "sveltestrap";

  import Transition from "../components/Transition.svelte";
  import { signup } from "../mutation.js";

  let passwordLowercaseRegex = /(?=.*[a-z])/;
  let passwordUppercaseRegex = /(?=.*[A-Z])/;
  let passwordNumberRegex = /(?=.*[0-9])/;
  let passwordSpecialCharacterRegex = /(?=.*[ !"#$%&'()*+,./:;<=>?@^_`{|}~\[\]\\-])/;
  let passwordLengthRegex = /(?=.{8,24})/;

  let username = "";
  let passwordOne = "";
  let passwordTwo = "";
  let errorMessage = "";

  $: usernameInvalid = errorMessage == "username already exists";
  $: passwordLowercaseInvalid = !passwordLowercaseRegex.test(passwordOne);
  $: passwordUppercaseInvalid = !passwordUppercaseRegex.test(passwordOne);
  $: passwordNumberInvalid = !passwordNumberRegex.test(passwordOne);
  $: passwordSpecialCharacterInvalid = !passwordSpecialCharacterRegex.test(passwordOne);
  $: passwordLengthInvalid = !passwordLengthRegex.test(passwordOne);
  $: passwordValid =
    !passwordLowercaseInvalid &&
    !passwordUppercaseInvalid &&
    !passwordNumberInvalid &&
    !passwordSpecialCharacterInvalid &&
    !passwordLengthInvalid;
  $: passwordMismatch = passwordTwo && passwordTwo != passwordOne;

  async function onSubmit(event) {
    event.preventDefault();
    if (passwordOne == passwordTwo) {
      try {
        await signup(username, passwordOne);
        goto("/", { replaceState: false });
      } catch (error) {
        errorMessage = error.response.errors[0].message;
      }
    } else {
      errorMessage = "password mismatch";
    }
  }
</script>

<Transition>
  <Row>
    <Col class="d-inline-flex">
      <Card class="mx-auto text-center" style="min-width: 33em">
        <CardHeader>
          <CardTitle class="mb-0">Enregistrement</CardTitle>
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
                  name="password1"
                  id="password1"
                  placeholder="Mot de passe"
                  bind:valid={passwordValid}
                  bind:value={passwordOne}
                  required
                />
              </InputGroup>
              <ListGroup flush>
                <ListGroupItem class={passwordLowercaseInvalid ? "text-danger" : "text-success"}>
                  Au moins une minuscule
                  {#if passwordLowercaseInvalid}
                    <Fa icon={faTimes} />
                  {:else}
                    <Fa icon={faCheck} />
                  {/if}
                </ListGroupItem>
                <ListGroupItem class={passwordUppercaseInvalid ? "text-danger" : "text-success"}>
                  Au moins une majuscule
                  {#if passwordUppercaseInvalid}
                    <Fa icon={faTimes} />
                  {:else}
                    <Fa icon={faCheck} />
                  {/if}
                </ListGroupItem>
                <ListGroupItem class={passwordNumberInvalid ? "text-danger" : "text-success"}>
                  Au moins un chiffre
                  {#if passwordNumberInvalid}
                    <Fa icon={faTimes} />
                  {:else}
                    <Fa icon={faCheck} />
                  {/if}
                </ListGroupItem>
                <ListGroupItem class={passwordSpecialCharacterInvalid ? "text-danger" : "text-success"}>
                  Au moins un caractère spécial
                  {#if passwordSpecialCharacterInvalid}
                    <Fa icon={faTimes} />
                  {:else}
                    <Fa icon={faCheck} />
                  {/if}
                </ListGroupItem>
                <ListGroupItem class={passwordLengthInvalid ? "text-danger" : "text-success"}>
                  Entre 8 et 24 caractères
                  {#if passwordLengthInvalid}
                    <Fa icon={faTimes} />
                  {:else}
                    <Fa icon={faCheck} />
                  {/if}
                </ListGroupItem>
              </ListGroup>
            </FormGroup>
            <FormGroup>
              <InputGroup>
                <InputGroupText>
                  <Fa icon={faKey} />
                </InputGroupText>
                <Input
                  type="password"
                  name="password2"
                  id="password2"
                  placeholder="Mot de passe"
                  bind:invalid={passwordMismatch}
                  bind:value={passwordTwo}
                  required
                />
              </InputGroup>
            </FormGroup>
            <FormGroup>
              <Button
                block
                color="dark"
                type="submit"
                disabled={!username || !passwordOne || !passwordTwo || !passwordValid || passwordMismatch}
                >Valider</Button
              >
            </FormGroup>
          </Form>
        </CardBody>
        <CardFooter>
          <Button block color="dark" href="/signin">
            Connexion
            <Fa icon={faArrowRight} />
          </Button>
        </CardFooter>
      </Card>
    </Col>
  </Row>
</Transition>
