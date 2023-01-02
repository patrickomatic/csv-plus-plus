require 'template'

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
        expect(template.rows[0].cells[1].modifier.align).to eq(['right'])
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
        expect(template.rows[0].modifier.align).to eq(['center'])
        expect(template.rows[0].modifier).to be_bold
      end
    end
  end

  describe "#interpolate_variables!" do
    # XXX 
  end

  describe "#parse_rows!" do
    context "with multiple infinite expands" do
      it "throws a SyntaxError" do
      # XXX
      end
    end
  end

  describe "#expand_rows!" do
    let(:template) { CSVPlusPlus::Template.new(rows: [ ])}
    let(:input) { "![[expand=2/format=bold]]foo,bar,baz\n![[expand]]" }

    it "updates row.index" do
      # XXX 
    end

    it "expands the rows to SPREADSHEET_INFINITY" do
      #expect(template.rows.length).to eq 1000
    end
  end
end
