import('./pkg')
    .then(module => {
        module.main(window.location.pathname);
    })
    .catch(console.error);