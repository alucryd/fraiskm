<script lang="ts">
  import { faAt, faKey } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import {
    Button,
    Form,
    FormGroup,
    Input,
    InputGroup,
    InputGroupText,
    Modal,
    ModalBody,
    ModalHeader,
  } from "sveltestrap";

  import { updateUsername } from "../mutation.js";

  export let isOpen = false;
  export let toggle = undefined;

  let newUsername = "";
  let password = "";
  let errorMessage = "";

  $: usernameInvalid = errorMessage == "username already exists";
  $: passwordInvalid = errorMessage == "invalid password";

  async function onSubmit(event) {
    event.preventDefault();
    try {
      await updateUsername(newUsername, password);
      isOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  }
</script>

<Modal {isOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Modification de l'adresse email</ModalHeader>
  <ModalBody class="pb-0">
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
            placeholder="Nouvelle adresse email"
            feedback="L'adresse email existe déjà"
            bind:invalid={usernameInvalid}
            bind:value={newUsername}
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
        <Button block color="dark" type="submit" disabled={!newUsername || !password}>Valider</Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
