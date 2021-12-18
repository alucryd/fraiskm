<script lang="ts">
  import { goto } from "$app/navigation";
  import { faGithub } from "@fortawesome/free-brands-svg-icons";
  import {
    faBug,
    faCar,
    faHeart,
    faMapMarkerAlt,
    faSignOutAlt,
    faTachometerAlt,
    faUserCog,
    faUserEdit,
    faUsers,
  } from "@fortawesome/free-solid-svg-icons";
  import "bootstrap/dist/css/bootstrap.min.css";
  import { onMount } from "svelte";
  import Fa from "svelte-fa";
  import {
    Collapse,
    Container,
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
    Nav,
    NavItem,
    NavLink,
    Navbar,
    NavbarBrand,
    NavbarToggler,
  } from "sveltestrap";

  import PasswordModal from "../components/PasswordModal.svelte";
  import UsernameModal from "../components/UsernameModal.svelte";
  import { signout } from "../mutation.js";
  import { getAddresses, getDrivers, getVehicles, me } from "../query.js";
  import { user } from "../store.js";

  let isUpperNavOpen = false;

  function handleUpperNavUpdate(event) {
    isUpperNavOpen = event.detail.isOpen;
  }

  let isLowerNavOpen = false;

  function handleLowerNavUpdate(event) {
    isLowerNavOpen = event.detail.isOpen;
  }

  let isUsernameModalOpen = false;
  const toggleUsernameModal = () => (isUsernameModalOpen = !isUsernameModalOpen);

  let isPasswordModalOpen = false;
  const togglePasswordModal = () => (isPasswordModalOpen = !isPasswordModalOpen);

  const onSignout = async (event) => {
    event.preventDefault();
    await signout();
    goto("/signin", { replaceState: false });
  };

  onMount(async () => {
    console.log($user);
    if (!$user) {
      try {
        await me();
        await getAddresses();
        await getVehicles();
        await getDrivers();
      } catch (error) {
        goto("/signin", { replaceState: true });
      }
    }
  });
</script>

<div class="d-flex flex-column min-vh-100">
  <Navbar color="dark" dark expand="md" class="sticky-top mb-3">
    <NavbarBrand href="/">
      <Fa icon={faTachometerAlt} />
      fraiskm
    </NavbarBrand>
    {#if $user}
      <NavbarToggler on:click={() => (isUpperNavOpen = !isUpperNavOpen)} />
      <Collapse isOpen={isUpperNavOpen} navbar expand="md" on:update={handleUpperNavUpdate}>
        <Nav navbar class="me-auto">
          <NavItem>
            <NavLink href="/addresses">
              <Fa icon={faMapMarkerAlt} />
              Mes adresses
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink href="/vehicles">
              <Fa icon={faCar} />
              Mes véhicules
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink href="/drivers">
              <Fa icon={faUsers} />
              Mes conducteurs
            </NavLink>
          </NavItem>
        </Nav>
        <Nav navbar class="ms-auto">
          <Dropdown nav inNavbar>
            <DropdownToggle nav>
              <Fa icon={faUserCog} />
            </DropdownToggle>
            <DropdownMenu end class="end-0">
              <DropdownItem on:click={toggleUsernameModal}>
                <Fa icon={faUserEdit} />
                Adresse email
              </DropdownItem>
              <DropdownItem on:click={togglePasswordModal}>
                <Fa icon={faUserEdit} />
                Mot de passe
              </DropdownItem>
              <DropdownItem divider />
              <DropdownItem on:click={onSignout}>
                <Fa icon={faSignOutAlt} />
                Déconnexion
              </DropdownItem>
            </DropdownMenu>
          </Dropdown>
        </Nav>
      </Collapse>
    {/if}
  </Navbar>

  <Container fluid class="flex-grow-1">
    <slot />
  </Container>

  <UsernameModal isOpen={isUsernameModalOpen} toggle={toggleUsernameModal} />
  <PasswordModal isOpen={isPasswordModalOpen} toggle={togglePasswordModal} />

  <Navbar color="dark" dark expand="md" class="mt-3">
    <NavbarBrand href="/">Copyright &copy; 2021 Maxime Gauduin</NavbarBrand>
    <NavbarToggler on:click={() => (isLowerNavOpen = !isLowerNavOpen)} />
    <Collapse isOpen={isLowerNavOpen} navbar expand="md" on:update={handleLowerNavUpdate}>
      <Nav navbar class="ms-auto">
        <NavItem>
          <NavLink href="https://github.com/alucryd/fraiskm">
            <Fa icon={faGithub} />
            Contribuer
          </NavLink>
        </NavItem>
        <NavItem>
          <NavLink href="https://github.com/alucryd/fraiskm/issues">
            <Fa icon={faBug} />
            Signaler
          </NavLink>
        </NavItem>
        <NavItem>
          <NavLink href="https://github.com/sponsors/alucryd">
            <Fa icon={faHeart} />
            Sponsoriser
          </NavLink>
        </NavItem>
      </Nav>
    </Collapse>
  </Navbar>
</div>

<style>
  :global(thead) {
    display: none !important;
  }

  :global(tfoot) {
    display: none !important;
  }

  :global(.table > :not(:first-child)) {
    border-top-width: 1px !important;
    border-top-color: inherit !important;
  }
</style>
