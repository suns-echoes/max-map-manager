--------------------------------------------------------------------------------
Dealer in Worlds                   Tech Support: [M.A.X. Port Discord]
                  email: dealer_in_wor0xffee000001
--------------------------------------------------------------------------------
Non Tech Support Questions/Comments send [M.A.X. Port Discord / max-map-manager]
--------------------------------------------------------------------------------



M.M.M.   RELEASE NOTES  v.0.9.5


       TABLE of CONTENTS

    (1) Requirements
    (2) Features
    (3) Instructions
    (4) Credits
    (5) Source Code License (MIT)
    (6) End User License Agreement (EULA)
    (7) Disclaimer of Warranty (Limited Warranty)
    (8) Contact & Contributions


--------------------------------------------------------------------------------
(1)   Requirements
--------------------------------------------------------------------------------

REQUIRED:

        - M.M.M. requires original game to be installed
        - Linux Debian 13 or newer
        - Linux Ubuntu 22 or newer
        - Windows 10 or newer

--------------------------------------------------------------------------------
(2)    FEATURES
--------------------------------------------------------------------------------

A utility for managing, archiving, and deploying map files and associated
save states for Mechanized Assault & eXploration (M.A.X.).

Technical Features

    Easy Map Management: Indexing and organization of M.A.X. map assets.

    Automated Archival: Synchronized backup of .WRL files and their
                        corresponding save game files into a unified archive.

    Restoration (Installation): Deploys archived maps and associated save states
                                to the active game directory for immediate use.

    Map Importing: Support for importing external map files into the game.

Planned Updates

    Metadata Editing: Interface for modifying map metadata information,
                      like map name, author, creation date, ...

    Direct-to-Archive Import: Mechanism to import files directly into
                              the archive.

    Batch Processing: Support for importing multiple at once.

--------------------------------------------------------------------------------
(3)    INSTRUCTIONS
--------------------------------------------------------------------------------

Prerequisites

    M.A.X. Installation: A valid installation of the original game is required.

    Directory Preparation: Create an empty folder to serve as your map archive
                           before launching the software.

Initial Setup

    Upon first execution, the following paths must be configured:

        M.A.X. Installation Directory: The root folder of the game.
                                       It must contain the MAX.RES
                                       file for verification.

        Save Files Directory: The directory where the game manages state data.
                              This is typically the root M.A.X. folder
                              or the MAX Port directory.

        Archive Directory: The dedicated folder where MMM will store
                           and index archived maps and their associated
                           save files.

UI Functionality

    Header Section

        IMPORT: Select an external map file via the file picker for import.

            Logic: The map is immediately installed into the currently selected
                   in-game map slot.

            Auto-Archival: If the target slot is occupied, the existing map
                           and its related saves are automatically moved
                           to the archive before the new map is deployed.

            Validation: Broken or incompatible files will trigger
                        an error message and abort the process.

        ARCHIVE (Toggle): Switches the display between
                          the Main View (Active Game Slots) and
                          the Archive View (Stored Maps).

        ? (Info): Displays technical information and software configuration.

    Main View (Active Game Slots)

        ARCHIVE: Manually moves the selected map and all associated save files
                 from the game directory to the archive.

        REPLACE: Switches context to the Archive View to select a specific map
                 for deployment into the active slot.

    Archive View (Stored Maps)

        INSTALL: Swaps the map currently selected in the Main View
                 with the chosen archived map.

                 This operation also moves the linked save states.

Special Codes

	When "/" is pressed, the input box will appear top-left side of the app.
	You can than enter the following codes:

	/SETUP

		Shows setup screen.

	/RELOAD

		Reloads the UI.

	/NOC._P{0xf93753}/missing/data/...

		IFf0-ge4 tr340F[ t5.gre>* #r433&dfe^


--------------------------------------------------------------------------------
(4)    CREDITS
--------------------------------------------------------------------------------

App Design:         Aneta Suns

Programmer:         Aneta Suns

2D Artists:         Aneta Suns

Testers:            klei1984
					delairec
                    Aneta Suns

    If you want to be mentiond here, please report bug!

--------------------------------------------------------------------------------
(5)     SOURCE CODE LICENSE (MIT)
--------------------------------------------------------------------------------

MIT License

Copyright (c) 2025 Aneta Suns

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the “Software”),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.

--------------------------------------------------------------------------------
(6)     APPLICATION END USER LICENSE AGREEMENT (EULA)
--------------------------------------------------------------------------------

Product:

    M.A.X. Map Manager

Copyright Holder:

    Aneta Suns

License & Distribution:

    This software is provided as Freeware.

    You are granted a non-exclusive right to use this software
    for any personal or non-commercial purpose.

You are encouraged to:

    Make copies of the software for backup or archival purposes.

    Distribute, share, and give away the original,
    unmodified compiled software to others free of charge.

Restrictions:

    You may not sell, rent, or lease the compiled software for profit.

    If you distribute the software, you must include this original copyright
    notice and license.

--------------------------------------------------------------------------------
(7)     DISCLAIMER OF WARRANTY (LIMITED WARRANTY)
--------------------------------------------------------------------------------

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

--------------------------------------------------------------------------------
(8)     CONTACT & CONTRIBUTIONS
--------------------------------------------------------------------------------

For bug reports or technical inquiries, please see:

    https://discord.com/channels/1085572457466974248/1390749356222709790

    or

    https://github.com/suns-echoes/max-map-manager/issues

--EOF--
