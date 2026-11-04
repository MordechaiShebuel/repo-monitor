BEGIN;

INSERT INTO application (name)
VALUES ('hyprland'), ('librewolf'), ('xlibre')
ON CONFLICT (name) DO NOTHING;

INSERT INTO endpoint (app_id, url)
SELECT a.id, v.url
FROM (
    VALUES
        (
            'hyprland',
            'https://github.com/hyprwm/Hyprland/releases/download/v${version}/source-v${version}.tar.gz'
        ),
        (
            'librewolf',
            'https://librewolf.dev/api/packages/librewolf/generic/librewolf-source/154.0-2/librewolf-154.0-2.source.tar.gz'
        ),
        (
            'xlibre',
            'https://github.com/X11Libre/xserver/wiki'
        )
) AS v(name, url)
JOIN application a ON a.name = v.name
ON CONFLICT (app_id, url) DO NOTHING;

INSERT INTO app_version (app_id, version)
SELECT a.id, v.version
FROM (
    VALUES
        ('hyprland', '0.56.2'),
        ('librewolf', '154.0.2'),
        ('xlibre', '25.1.0')
) AS v(name, version)
JOIN application a ON a.name = v.name
ON CONFLICT (app_id)
DO UPDATE SET version = EXCLUDED.version, updated_at = now();

COMMIT;
