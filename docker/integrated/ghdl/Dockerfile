# Dock: Create a docker image containing tools and programs required
#   to run VHDL simulations.

# Stage 0: Orbit layer (1st step)
FROM cdotrus/orbit:latest-ubuntu-22.04

LABEL maintainer="Chase Ruskin"

# Stage 1: Python layer
FROM brunneis/python:3.8.6-ubuntu-20.04

# Configure Python for any scripting during simulations
RUN python -m pip install --upgrade pip

COPY --from=0 / /

# Stage 2: FPGA Toolchain layer (GHDL simulator)
FROM cdotrus/ghdl-mcode:latest-ubuntu-22.04

COPY --from=1 / /

# Enter the shell
CMD ["bash"]