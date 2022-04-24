<script lang="ts">
  import { faBriefcase, faFont, faHome, faMapMarkerAlt, faSearch } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import {
    Button,
    ButtonGroup,
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
    Tooltip,
  } from "sveltestrap";

  import { createAddress, normalizeAddress, updateAddress } from "../mutation.js";
  import { getAddresses } from "../query.js";
  import { isAddressModalOpen } from "../store.js";

  export let toggle = undefined;
  export let address = undefined;

  let normalizedAddresses = [];

  let errorMessage = "";

  const setAddressType = (event, int) => {
    event.preventDefault();
    address.addressType = int;
  };

  const onSearch = async (event) => {
    event.preventDefault();
    normalizedAddresses = await normalizeAddress(address.label);
  };

  const setLabel = (label) => {
    address.label = label;
    normalizedAddresses = [];
  };

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      if (address.id) {
        await updateAddress(address);
      } else {
        await createAddress(address);
      }
      await getAddresses();
      $isAddressModalOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Modal isOpen={$isAddressModalOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Adresse</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faFont} />
          </InputGroupText>
          <Input type="text" name="title" id="title" placeholder="Titre" bind:value={address.title} required />
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faMapMarkerAlt} />
          </InputGroupText>
          <Input type="text" name="label" id="label" placeholder="Libellé" bind:value={address.label} required />
          <Button color="dark" on:click={onSearch}>
            <Fa icon={faSearch} />
          </Button>
        </InputGroup>
        <Collapse isOpen={Boolean(normalizedAddresses)}>
          <ListGroup flush>
            {#each normalizedAddresses as address}
              <ListGroupItem tag="button" action on:click={() => setLabel(address)}>{address}</ListGroupItem>
            {/each}
          </ListGroup>
        </Collapse>
      </FormGroup>
      <FormGroup>
        <ButtonGroup class="w-100">
          <Button
            block
            color={address.addressType == 0 ? "dark" : "secondary"}
            id="address-type-0"
            on:click={(event) => setAddressType(event, 0)}
          >
            <Fa icon={faHome} />
          </Button>
          <Tooltip target="address-type-0" placement="top">Personnelle</Tooltip>
          <Button
            block
            color={address.addressType == 1 ? "dark" : "secondary"}
            id="address-type-1"
            on:click={(event) => setAddressType(event, 1)}
          >
            <Fa icon={faBriefcase} />
          </Button>
          <Tooltip target="address-type-1" placement="top">Professionnelle</Tooltip>
        </ButtonGroup>
      </FormGroup>
      <FormGroup>
        <Button block color="dark" type="submit" disabled={!address.title || !address.label}>Valider</Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
