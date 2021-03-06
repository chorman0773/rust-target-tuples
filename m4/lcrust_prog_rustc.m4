
AC_DEFUN([LCRUST_PROG_RUSTC],[
    AC_REQUIRE([AC_PROG_CC])
    AC_REQUIRE([AC_CANONICAL_HOST])
    AC_ARG_VAR(RUSTC,[Rust compiler to use])
    AC_ARG_VAR(RUSTFLAGS,[Flags to pass to the rust compiler])

    if test "$RUSTFLAGS" \= "" 
    then
        RUSTFLAGS="-C opt-level=2 -g"
    fi

    if test x$host_alias != x 
    then
        AC_PATH_PROGS(RUSTC,[rustc lcrustc $host-gccrs])
    else 
        AC_PATH_PROGS(RUSTC,[rustc lcrustc $host-gccrs gccrs])
    fi

    if test "$RUSTC" \= ""
    then
        AC_MSG_ERROR([Failed to find a rust compiler. Install rustc in PATH, or set RUSTC to a suitable compiler])
    fi

    if test x$host_alias != x
    then
        case $RUSTC in 
            *[\\/]$host-* ) dnl gccrs has a host prefix when cross-compiling, so no need to attempt using `--target`
                ;;
            * )
                SAVE_RUSTFLAGS="$RUSTFLAGS"
                AC_MSG_CHECKING([how to cross compile with $RUSTC])
                RUSTFLAGS="$RUSTFLAGS --target $host"
                echo '' > test.rs
                $RUSTC $RUSTFLAGS --crate-type rlib --crate-name test test.rs 2>> config.log > /dev/null
                
                if test $? -eq 0
                then
                    AC_MSG_RESULT([--target $host])
                else
                    rm -f test.rs libtest.rlib
                    RUSTFLAGS="$SAVE_RUSTFLAGS --target $host_alias"
                    echo '' > test.rs
                    $RUSTC $RUSTFLAGS --crate-type rlib --crate-name test test.rs  2>> config.log > /dev/null
                    if test $? -eq 0
                    then
                        rm -f test.rs libtest.rlib
                        AC_MSG_RESULT([--target $host_alias])
                    else
                        rm -f test.rs libtest.rlib
                        case "$host" in 
                            x86_64-pc-*-* )
                                IFS="-" read arch vendor kernel env  <<< "$host"
                                host_target="$arch-unknown-$kernel-$env"
                                RUSTFLAGS="$SAVE_RUSTFLAGS --target $host_target"
                                echo '' > test.rs
                                $RUSTC $RUSTFLAGS --crate-type rlib --crate-name test test.rs  2>> config.log > /dev/null
                                if test $? -eq 0
                                then
                                    rm -f test.rs libtest.rlib
                                    AC_MSG_RESULT([--target $host_target])
                                else
                                    AC_MSG_RESULT([failed])
                                    AC_MSG_ERROR([Cannot determine how to cross compile to $host with $RUSTC])
                                fi
                            ;;

                            i?86-pc-*-* )
                                IFS="-" read arch vendor kernel env  <<< "$host"
                                host_target="$arch-unknown-$kernel-$env"
                                RUSTFLAGS="$SAVE_RUSTFLAGS --target $host_target"
                                echo '' > test.rs
                                $RUSTC $RUSTFLAGS --crate-type rlib --crate-name test test.rs  2>> config.log > /dev/null
                                if test $? -eq 0
                                then
                                    rm -f test.rs libtest.rlib
                                    AC_MSG_RESULT([--target $host_target])
                                else
                                    AC_MSG_RESULT([failed])
                                    AC_MSG_ERROR([Cannot determine how to cross compile to $host with $RUSTC])
                                fi
                            ;;

                            *)
                                AC_MSG_RESULT([failed])
                                AC_MSG_ERROR([Cannot determine how to cross compile to $host with $RUSTC])
                                ;;
                        esac
                    fi
                fi
                ;;
        esac
    fi

    AC_MSG_CHECKING([whether Rust compiler works])
    echo 'fn main(){}' > test.rs 
    $RUSTC $RUSTFLAGS --crate-type bin --crate-name test test.rs 2>> config.log > /dev/null
    if test $? -ne 0
    then
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([Cannot compile a simple program with $RUSTC])
    fi

    if test x$host_alias != x 
    then
        ./test${EXEEXT}
        if test $? -ne 0
        then
            AC_MSG_RESULT([no])
            AC_MSG_ERROR([Cannot run executables compiled by $RUSTC])
        fi
    fi

    rm -rf test.rs test${EXEEXT}

    AC_MSG_RESULT([yes])

    AC_SUBST(RUSTC)
    AC_SUBST(RUSTFLAGS)
])

AC_DEFUN([LCURST_RUSTC_VERSION],[
    AC_REQUIRE([LCRUST_PROG_RUSTC])

    version_output="`${RUSTC} --version`"

    AC_MSG_CHECKING(the rust version supported by ${RUSTC})
    
    read rustc_name rust_version <<< ${version_output}

    AC_MSG_RESULT(${rust_version})

    case $rust_version in
        *.*.*-beta.* )
            rust_channel=beta
            IFS="." read rust_major rust_minor _lcrust_rest <<< ${version_output}
            IFS="-" read rust_patch <<< ${_lcrust_rest}
            ;;
        *.*.*-* )
            IFS="." read rust_major rust_minor _lcrust_rest <<< ${version_output}
            IFS="-" read rust_patch rust_channel <<< ${_lcrust_rest}
            ;;
        *.*.* )
            rust_channel=stable
            IFS="." read rust_major rust_minor rust_patch <<< ${version_output}
            ;;
    esac
    AC_MSG_CHECKING(whether $RUSTC is lccc)
    case $name in
        lcrust* | lccc* ) dnl lccc doesn't distinguish between stable and unstable compiler, 
            rustc_is_lccc=yes
            ;;
        * )
            rustc_is_lccc=no
            ;;
    esac
    AC_MSG_RESULT([$rustc_is_lccc])
    
    AC_SUBST(rustc_name)
    AC_SUBST(rust_version)
    AC_SUBST(rust_channel)
    AC_SUBST(rustc_is_lccc)
])

AC_DEFUN([LCRUST_PROG_RUSTC_FOR_BUILD],[
    AC_REQUIRE([AX_PROG_CC_FOR_BUILD])
    AC_REQUIRE([AC_CANONICAL_BUILD])
    AC_ARG_VAR(RUSTC_FOR_BUILD,[Rust compiler to use on the build system])
    AC_ARG_VAR(RUSTFLAGS_FOR_BUILD,[Flags to pass to the rust compiler for the build system])

    AC_MSG_NOTICE([checking for the compiler to use for $build...])

    AC_PATH_PROGS(RUSTC_FOR_BUILD,[rustc lcrustc $build-gccrs gccrs])

    if test "$RUSTC_FOR_BUILD" \= ""
    then
        AC_MSG_NOTICE([checking for the compiler to use for $build... not found])
        AC_MSG_ERROR([Failed to find a rust compiler. Install rustc in PATH, or set RUSTC_FOR_BUILD to a suitable compiler])
    fi

    AC_MSG_NOTICE([checking for the compiler to use for $build... $RUSTC_FOR_BUILD])


    AC_MSG_CHECKING([whether Rust compiler works])
    echo 'fn main(){}' > test.rs 
    $RUSTC_FOR_BUILD $RUSTFLAGS_FOR_BUILD --crate-type bin --crate-name test test.rs 2>> config.log > /dev/null
    if test $? -ne 0
    then
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([Cannot compile a simple program with $RUSTC_FOR_BUILD])
    fi
    
    ./test${EXEEXT_FOR_BUILD}
    if test $? -ne 0
    then
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([Cannot run executables compiled by $RUSTC_FOR_BUILD])
    fi

    rm -rf test.rs test${EXEEXT_FOR_BUILD}

    AC_MSG_RESULT([yes])

    AC_SUBST(RUSTC_FOR_BUILD)
    AC_SUBST(RUSTFLAGS_FOR_BUILD)
])

AC_DEFUN([LCRUST_TRY_COMPILE],[
    echo '$1' >> test.rs
    ${RUSTC} ${RUSTFLAGS} --crate-type rlib --crate-name test --emit link=libtest.rlib test.rs

    if test $? -eq 0 
    then
        rm -f test.rs libtest.rlib
        $2
    else
        rm -f test.rs libtest.rlib
        $3
    fi
])
