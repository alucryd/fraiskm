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
  import { user } from "../store.js";

  let isNavOpen = false;
  let isUsernameModalOpen = false;
  let isPasswordModalOpen = false;

  function handleUpdate(event) {
    isNavOpen = event.detail.isOpen;
  }

  const toggleUsernameModal = () => (isUsernameModalOpen = !isUsernameModalOpen);
  const togglePasswordModal = () => (isPasswordModalOpen = !isPasswordModalOpen);

  async function onSignout(event) {
    event.preventDefault();
    await signout();
    goto("/signin", { replaceState: false });
  }
</script>

<div class="d-flex flex-column min-vh-100">
  <Navbar color="dark" dark expand="md" class="sticky-top mb-3">
    <NavbarBrand href="/">
      <Fa icon={faTachometerAlt} />
      fraiskm
    </NavbarBrand>
    {#if $user}
      <NavbarToggler on:click={() => (isNavOpen = !isNavOpen)} />
      <Collapse isOpen={isNavOpen} navbar expand="md" on:update={handleUpdate}>
        <Nav navbar class="me-auto">
          <NavItem>
            <NavLink href="/drivers">
              <Fa icon={faUsers} />
              Mes conducteurs
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink href="/vehicles">
              <Fa icon={faCar} />
              Mes véhicules
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink href="/adresses">
              <Fa icon={faMapMarkerAlt} />
              Mes adresses
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
    <NavbarBrand href="/">Copyright © 2021 Maxime Gauduin</NavbarBrand>
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
  </Navbar>
</div>
