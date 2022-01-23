Chomp.registerTemplate('prettier', function ({ name, targets, deps, env, templateOptions: { files = '.', check = false, write = true, config = null, noErrorOnUnmatchedPattern = false, autoInstall } }, { CHOMP_EJECT }) {
  return [{
    name,
    targets,
    deps: [...deps, ...CHOMP_EJECT ? [] : ['node_modules/prettier']],
    invalidation: 'always',
    env,
    run: `prettier ${files} ${
        check ? ' --check' : ''
      }${
        write ? ' --write' : ''
      }${
        config ? ` --config ${config}` : ''
      }${
        noErrorOnUnmatchedPattern ? ' --no-error-on-unmatched-pattern' : ''
      }`
  }, ...CHOMP_EJECT ? [] : [{
    template: 'npm',
    templateOptions: {
      autoInstall,
      packages: ['prettier'],
      dev: true
    }
  }]];
});