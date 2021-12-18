<script lang="ts">
  import { faCheck, faKey, faTimes } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import {
    Button,
    Collapse,
    Form,
    FormGroup,
    Input,
    InputGroup,
    InputGroupText,
    ListGroup,
    ListGroupItem,
    Modal,
    ModalBody,
    ModalHeader,
  } from "sveltestrap";

  import {
    passwordLengthRegex,
    passwordLowercaseRegex,
    passwordNumberRegex,
    passwordSpecialCharacterRegex,
    passwordUppercaseRegex,
    updatePassword,
  } from "../mutation.js";

  export let isOpen = false;
  export let toggle = undefined;

  let password = "";
  let newPasswordOne = "";
  let newPasswordTwo = "";

  let errorMessage = "";

  $: passwordInvalid = errorMessage == "invalid password";
  $: newPasswordLowercaseInvalid = !passwordLowercaseRegex.test(newPasswordOne);
  $: newPasswordUppercaseInvalid = !passwordUppercaseRegex.test(newPasswordOne);
  $: newPasswordNumberInvalid = !passwordNumberRegex.test(newPasswordOne);
  $: newPasswordSpecialCharacterInvalid = !passwordSpecialCharacterRegex.test(newPasswordOne);
  $: newPasswordLengthInvalid = !passwordLengthRegex.test(newPasswordOne);
  $: newPasswordValid =
    !newPasswordLowercaseInvalid &&
    !newPasswordUppercaseInvalid &&
    !newPasswordNumberInvalid &&
    !newPasswordSpecialCharacterInvalid &&
    !newPasswordLengthInvalid;
  $: newPasswordMismatch = newPasswordTwo && newPasswordTwo != newPasswordOne;

  async function onSubmit(event) {
    event.preventDefault();
    try {
      await updatePassword(password, newPasswordOne);
      isOpen = false;
      password = "";
      newPasswordOne = "";
      newPasswordTwo = "";
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  }
</script>

<Modal {isOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Modification du mot de passe</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
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
        <InputGroup>
          <InputGroupText>
            <Fa icon={faKey} />
          </InputGroupText>
          <Input
            type="password"
            name="newpassword1"
            id="newpassword1"
            placeholder="Nouveau mot de passe"
            bind:valid={newPasswordValid}
            bind:value={newPasswordOne}
            required
          />
        </InputGroup>
        <Collapse isOpen={Boolean(newPasswordOne)}>
          <ListGroup flush>
            <ListGroupItem class={newPasswordLowercaseInvalid ? "text-danger" : "text-success"}>
              Au moins une minuscule
              {#if newPasswordLowercaseInvalid}
                <Fa icon={faTimes} />
              {:else}
                <Fa icon={faCheck} />
              {/if}
            </ListGroupItem>
            <ListGroupItem class={newPasswordUppercaseInvalid ? "text-danger" : "text-success"}>
              Au moins une majuscule
              {#if newPasswordUppercaseInvalid}
                <Fa icon={faTimes} />
              {:else}
                <Fa icon={faCheck} />
              {/if}
            </ListGroupItem>
            <ListGroupItem class={newPasswordNumberInvalid ? "text-danger" : "text-success"}>
              Au moins un chiffre
              {#if newPasswordNumberInvalid}
                <Fa icon={faTimes} />
              {:else}
                <Fa icon={faCheck} />
              {/if}
            </ListGroupItem>
            <ListGroupItem class={newPasswordSpecialCharacterInvalid ? "text-danger" : "text-success"}>
              Au moins un caractère spécial
              {#if newPasswordSpecialCharacterInvalid}
                <Fa icon={faTimes} />
              {:else}
                <Fa icon={faCheck} />
              {/if}
            </ListGroupItem>
            <ListGroupItem class={newPasswordLengthInvalid ? "text-danger" : "text-success"}>
              Entre 8 et 24 caractères
              {#if newPasswordLengthInvalid}
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
            name="newpassword2"
            id="newpassword2"
            placeholder="Nouveau mot de passe"
            bind:invalid={newPasswordMismatch}
            bind:value={newPasswordTwo}
            required
          />
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <Button
          block
          color="dark"
          type="submit"
          disabled={!password || !newPasswordOne || !newPasswordTwo || !newPasswordValid || newPasswordMismatch}
        >
          Valider
        </Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
