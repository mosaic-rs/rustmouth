/*
This file is part of RUSTMOUTH.

RUSTMOUTH is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

RUSTMOUTH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
RUSTMOUTH. If not, see <https://www.gnu.org/licenses/>.
*/

#define EXPORT __attribute__((visibility("default")))

void gsl_set_error_handler_off() { }

void _gsl_set_error_handler_off() { }

// math constants
double dlamch_(const char *cmach) { return 0.0; }
double dlamch(const char *cmach) { return 0.0; }
double NUMlapack_dlamch_(const char *cmach) { return 0.0; }