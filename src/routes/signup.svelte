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
    Collapse,
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
  import {
    passwordLengthRegex,
    passwordLowercaseRegex,
    passwordNumberRegex,
    passwordSpecialCharacterRegex,
    passwordUppercaseRegex,
    signup,
  } from "../mutation.js";

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
    try {
      await signup(username, passwordOne);
      goto("/", { replaceState: false });
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  }
</script>

<Transition>
  <Row class="justify-content-center">
    <Col class="col-lg-4 col-md-6 col-sm-12">
      <Card class="text-center">
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
                  feedback="L'adresse email existe déjà"
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
              <Collapse isOpen={Boolean(passwordOne)}>
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
              </Collapse>
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
              >
                Valider
              </Button>
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
