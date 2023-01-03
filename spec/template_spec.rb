require 'template'
require 'syntax_error'

describe CSVPlusPlus::Template do
  describe "#process!" do
    let(:template) { CSVPlusPlus::Template.process!(input) }
    let(:input) { "foo0,bar0,baz0\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    it "creates rows" do
      expect(template.rows.length).to eq 3
    end

    it "sets row.index" do
      expect(template.rows[0].index).to eq 0
      expect(template.rows[1].index).to eq 1
      expect(template.rows[2].index).to eq 2
    end

    context "with cell modifiers" do
      let(:input) { "foo,[[align=right/format=bold]]bar,baz" }

      it "creates cells with the modifiers" do
        expect(template.rows[0].cells[1].modifier).to be_right_align
        expect(template.rows[0].cells[1].modifier).to be_bold
      end
    end

    context "with cell modifiers with multiple values" do
      let(:input) { "foo,[[align=right/format=bold/format=italic]]bar,baz" }

      it "creates cells with the modifiers" do
        expect(template.rows[0].cells[1].modifier).to be_bold
        expect(template.rows[0].cells[1].modifier).to be_italic
      end
    end

    context "with row modifiers" do
      let(:input) { "![[align=center/format=bold]]foo,bar,baz" }

      it "creates rows with the modifiers" do
        expect(template.rows[0].modifier).to be_center_align
        expect(template.rows[0].modifier).to be_bold
      end
    end
  end

  describe "#interpolate_variables!" do
    let(:template) { CSVPlusPlus::Template.new(rows:, key_values:,) }
    let(:key_values) { {} }
    let(:cells_row0) do
      [
        CSVPlusPlus::Cell.new(0, 0, "=$$foo", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(0, 1, "=foo", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(0, 2, "foo", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(0, 4, "=$$rownum", CSVPlusPlus::Modifier.new),
      ]
    end
    let(:rows) { [CSVPlusPlus::Row.new(0, cells_row0, CSVPlusPlus::Modifier.new)] }

    before { template.interpolate_variables!({ "foo" => [:number, 1] }) }

    it "interpolates the first one and leaves the others alone" do
      expect(template.rows[0].cells[0].to_csv).to eq "=1"
      expect(template.rows[0].cells[1].to_csv).to eq "=foo"
      expect(template.rows[0].cells[2].to_csv).to eq "foo"
    end

    it "interpolates $$rownum" do
      expect(template.rows[0].cells[3].to_csv).to eq "=1"
    end

    context "with key_values should override builtins" do
      let(:key_values) { { "rownum" => "1111"} }

      it "interpolates and overrides $$rownum" do
        expect(template.rows[0].cells[3].to_csv).to eq "=1111"
      end
    end
  end

  describe "#parse_rows!" do
    let(:template) { CSVPlusPlus::Template.new }
    let(:input) do
      Tempfile.new.tap do |f|
        f.write(file_contents)
        f.rewind
      end
    end
    let(:file_contents) { "foo,bar,baz\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    after(:each) { input.close! }

    it "parses the CSV rows" do
      template.parse_rows! input
      expect(template.rows.length).to eq 3
    end

    context "with multiple infinite expands" do
      let(:file_contents) { "![[expand]]foo,bar,baz\n![[expand]]foo1,bar1,baz1\nfoo2,bar2,baz2\n" }

      it "throws a SyntaxError" do
        expect { template.parse_rows! input }.to raise_error(CSVPlusPlus::SyntaxError)
      end
    end
  end

  describe "#expand_rows!" do
    let(:template) { CSVPlusPlus::Template.new(rows:,) }
    let(:cells_row0) do
      [
        CSVPlusPlus::Cell.new(0, 0, "foo", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(0, 1, "foo", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(0, 2, "foo", CSVPlusPlus::Modifier.new),
      ]
    end
    let(:cells_row1) do
      [
        CSVPlusPlus::Cell.new(1, 0, "a", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(1, 1, "nother", CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(1, 2, "row", CSVPlusPlus::Modifier.new),
      ]
    end
    let(:rows) do
      [
        CSVPlusPlus::Row.new(0, cells_row0,
                             CSVPlusPlus::Modifier.new.tap {|m| m.expand = CSVPlusPlus::Modifier::Expand.new(2) }),
        CSVPlusPlus::Row.new(1, cells_row1, CSVPlusPlus::Modifier.new),
      ]
    end

    before { template.expand_rows! }

    it "updates row.index" do
      expect(template.rows[0].index).to eq 0
      expect(template.rows[0].cells[0].row_index).to eq 0
      expect(template.rows[0].cells[1].row_index).to eq 0
      expect(template.rows[0].cells[2].row_index).to eq 0
      expect(template.rows[1].index).to eq 1
      expect(template.rows[1].cells[0].row_index).to eq 1
      expect(template.rows[1].cells[1].row_index).to eq 1
      expect(template.rows[1].cells[2].row_index).to eq 1
      expect(template.rows[2].index).to eq 2
      expect(template.rows[2].cells[0].row_index).to eq 2
      expect(template.rows[2].cells[1].row_index).to eq 2
      expect(template.rows[2].cells[2].row_index).to eq 2
    end

    context "with an infinite expand" do
      let(:rows) do
        [
          CSVPlusPlus::Row.new(0, cells_row0, CSVPlusPlus::Modifier.new),
          CSVPlusPlus::Row.new(1, cells_row1,
                               CSVPlusPlus::Modifier.new.tap {|m| m.expand = CSVPlusPlus::Modifier::Expand.new }),
        ]
      end

      it "expands the rows to SPREADSHEET_INFINITY without repetitions" do
        expect(template.rows.length).to eq 1000
      end
    end
  end
end
